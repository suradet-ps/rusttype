//! Shareable result card: draw the finished session onto an off-screen
//! canvas and download it as a PNG.
//!
//! Colors are read from the `:root` design tokens at runtime because canvas
//! cannot use CSS variables; the fallbacks are neutral, so no brand hex is
//! duplicated here. The export follows the CodeFrame approach: a
//! full-resolution canvas, a font-load guard before drawing,
//! `toBlob("image/png")` and an object-URL download.

use engine::SessionStats;
use js_sys::{Function, Promise};
use snippets::Snippet;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{
  Blob, CanvasRenderingContext2d, CssStyleDeclaration, HtmlAnchorElement, HtmlCanvasElement, Url,
};

use crate::components::{HighlightData, get_highlight};

/// Logical card width in pixels (the canvas is drawn at [`SCALE`]).
const CARD_WIDTH: f64 = 760.0;
/// Canvas scale: exports are always 2x for crisp text.
const SCALE: f64 = 2.0;
/// Card padding on every side.
const PAD: f64 = 48.0;
/// Brand mark size.
const LOGO: f64 = 48.0;
/// Gap between the header block and the release title.
const TITLE_GAP: f64 = 48.0;
/// Snippet title font size.
const TITLE_SIZE: f64 = 22.0;
/// Gap between the title and the stats row.
const STATS_GAP: f64 = 32.0;
/// Stats value font size.
const STATS_VALUE_SIZE: f64 = 34.0;
/// Stats label font size.
const STATS_LABEL_SIZE: f64 = 12.0;
/// Gap between the stats labels and the divider rule.
const DIVIDER_GAP: f64 = 18.0;
/// Gap between the divider and the code panel.
const PANEL_GAP: f64 = 32.0;
/// Code panel interior padding.
const PANEL_PAD: f64 = 24.0;
/// Excerpt font size.
const CODE_SIZE: f64 = 16.0;
/// Excerpt line height.
const CODE_LINE_HEIGHT: f64 = 26.0;
/// Maximum excerpt lines drawn on the card.
const MAX_EXCERPT_LINES: usize = 8;
/// Maximum excerpt line width, in display columns.
const MAX_EXCERPT_CHARS: usize = 62;
/// Longest slug used in the download filename.
const MAX_SLUG: usize = 32;

/// Display font stack (matches the UI token).
const FONT_DISPLAY: &str = "\"Inter\", sans-serif";
/// Mono font stack (matches the UI token).
const FONT_MONO: &str = "\"JetBrains Mono\", monospace";

/// RGB color.
type Rgb = (u8, u8, u8);

/// Brand colors resolved from the document's CSS tokens.
#[derive(Clone, Copy)]
struct CardTheme {
  canvas: Rgb,
  canvas_soft: Rgb,
  ink: Rgb,
  body_mid: Rgb,
  mute: Rgb,
  primary: Rgb,
  on_primary: Rgb,
}

impl Default for CardTheme {
  /// Neutral fallbacks used only when a token cannot be read.
  fn default() -> Self {
    Self {
      canvas: (255, 255, 255),
      canvas_soft: (245, 245, 245),
      ink: (17, 17, 17),
      body_mid: (128, 128, 128),
      mute: (200, 200, 200),
      primary: (200, 70, 0),
      on_primary: (255, 255, 255),
    }
  }
}

/// One same-class run inside an excerpt line.
#[derive(Debug, PartialEq)]
struct CodeRun {
  /// Syntax class index into [`HighlightData::colors`].
  class: usize,
  /// Text of the run, tabs already expanded.
  text: String,
}

/// Vertical layout (logical pixels) for a card with `excerpt_lines` lines.
#[derive(Debug, PartialEq)]
struct Layout {
  title_baseline: f64,
  stats_value_baseline: f64,
  stats_label_baseline: f64,
  divider_y: f64,
  panel_top: f64,
  panel_height: f64,
  card_height: f64,
}

/// Render the finished session as a PNG and start the download.
pub fn export_results_png(
  snippet: Snippet,
  stats: SessionStats,
  on_finished: impl FnOnce(Result<(), String>) + 'static,
) {
  spawn_local(async move {
    let result = render_and_download(&snippet, &stats).await;
    on_finished(result);
  });
}

/// Build the off-screen canvas, paint the card and download the blob.
async fn render_and_download(snippet: &Snippet, stats: &SessionStats) -> Result<(), String> {
  ensure_fonts_ready().await;

  let window = web_sys::window().ok_or_else(|| "no window object".to_string())?;
  let document = window
    .document()
    .ok_or_else(|| "no document object".to_string())?;

  let highlight = get_highlight(&snippet.code);
  let theme = read_theme();
  let canvas: HtmlCanvasElement = document
    .create_element("canvas")
    .map_err(js_err)?
    .unchecked_into();
  draw_card(&canvas, snippet, stats, &highlight, &theme)?;

  let blob = canvas_to_blob(&canvas).await?;
  let url = Url::create_object_url_with_blob(&blob).map_err(js_err)?;
  let anchor: HtmlAnchorElement = document
    .create_element("a")
    .map_err(js_err)?
    .unchecked_into();
  anchor.set_href(&url);
  anchor.set_download(&share_filename(&snippet.title, stats.wpm));
  anchor.click();
  // Revoking the object URL immediately after click() races the download in
  // Firefox; give the browser a moment to start.
  wait_ms(1500).await;
  let _ = Url::revoke_object_url(&url);
  Ok(())
}

/// Trigger and await every font variant the card draws with.
async fn ensure_fonts_ready() {
  let Some(document) = web_sys::window().and_then(|window| window.document()) else {
    return;
  };
  let fonts = document.fonts();
  for spec in [
    "400 16px \"Inter\"",
    "600 22px \"Inter\"",
    "700 28px \"Inter\"",
    "400 16px \"JetBrains Mono\"",
    "600 34px \"JetBrains Mono\"",
  ] {
    // A rejected future just means that variant does not exist.
    let _ = JsFuture::from(fonts.load(spec)).await;
  }
  if let Ok(ready) = fonts.ready() {
    let _ = JsFuture::from(ready).await;
  }
}

/// Paint the full card onto `canvas`.
fn draw_card(
  canvas: &HtmlCanvasElement,
  snippet: &Snippet,
  stats: &SessionStats,
  highlight: &HighlightData,
  theme: &CardTheme,
) -> Result<(), String> {
  let excerpt = excerpt_runs(
    &snippet.code,
    &highlight.char_classes,
    MAX_EXCERPT_LINES,
    MAX_EXCERPT_CHARS,
  );
  let layout = layout(excerpt.len());

  canvas.set_width((CARD_WIDTH * SCALE) as u32);
  canvas.set_height((layout.card_height * SCALE) as u32);
  let context = canvas
    .get_context("2d")
    .map_err(js_err)?
    .ok_or_else(|| "canvas 2d context unavailable".to_string())?;
  let ctx: CanvasRenderingContext2d = context
    .dyn_into()
    .map_err(|_| "unexpected canvas context".to_string())?;
  ctx.scale(SCALE, SCALE).map_err(js_err)?;

  draw_background(&ctx, &layout, theme)?;
  draw_header(&ctx, theme)?;
  draw_title(&ctx, &snippet.title, &layout, theme)?;
  draw_stats(&ctx, stats, &layout, theme)?;
  draw_excerpt(&ctx, &excerpt, highlight, &layout, theme)?;
  Ok(())
}

/// Card background and hairline border.
fn draw_background(
  ctx: &CanvasRenderingContext2d,
  layout: &Layout,
  theme: &CardTheme,
) -> Result<(), String> {
  ctx.set_fill_style_str(&rgb(theme.canvas));
  rounded_rect(
    ctx,
    0.5,
    0.5,
    CARD_WIDTH - 1.0,
    layout.card_height - 1.0,
    12.0,
  )?;
  ctx.fill();
  ctx.set_stroke_style_str(&rgb(theme.mute));
  ctx.set_line_width(1.0);
  ctx.stroke();
  Ok(())
}

/// Brand mark, wordmark, tagline and date.
fn draw_header(ctx: &CanvasRenderingContext2d, theme: &CardTheme) -> Result<(), String> {
  ctx.set_fill_style_str(&rgb(theme.primary));
  rounded_rect(ctx, PAD, PAD, LOGO, LOGO, 12.0)?;
  ctx.fill();

  ctx.set_fill_style_str(&rgb(theme.on_primary));
  ctx.set_font(&format!("700 28px {FONT_DISPLAY}"));
  ctx.set_text_align("center");
  ctx.set_text_baseline("middle");
  fill_text(ctx, "R", PAD + LOGO / 2.0, PAD + LOGO / 2.0 + 1.0)?;

  ctx.set_text_align("left");
  ctx.set_text_baseline("alphabetic");
  ctx.set_fill_style_str(&rgb(theme.ink));
  ctx.set_font(&format!("600 26px {FONT_DISPLAY}"));
  fill_text(ctx, "RustType", PAD + LOGO + 16.0, PAD + 24.0)?;

  ctx.set_fill_style_str(&rgb(theme.body_mid));
  ctx.set_font(&format!("400 14px {FONT_DISPLAY}"));
  fill_text(
    ctx,
    "Practice typing real code",
    PAD + LOGO + 17.0,
    PAD + 46.0,
  )?;

  ctx.set_text_align("right");
  ctx.set_font(&format!("400 13px {FONT_MONO}"));
  let date = format_date(js_sys::Date::now());
  fill_text(ctx, &date, CARD_WIDTH - PAD, PAD + 30.0)?;
  ctx.set_text_align("left");
  Ok(())
}

/// Snippet title above the stats.
fn draw_title(
  ctx: &CanvasRenderingContext2d,
  title: &str,
  layout: &Layout,
  theme: &CardTheme,
) -> Result<(), String> {
  ctx.set_fill_style_str(&rgb(theme.ink));
  ctx.set_font(&format!("600 {TITLE_SIZE}px {FONT_DISPLAY}"));
  let text = fit_text(ctx, title, CARD_WIDTH - PAD * 2.0);
  fill_text(ctx, &text, PAD, layout.title_baseline)
}

/// Stats row plus the divider above the code panel.
fn draw_stats(
  ctx: &CanvasRenderingContext2d,
  stats: &SessionStats,
  layout: &Layout,
  theme: &CardTheme,
) -> Result<(), String> {
  let entries = [
    ("WPM", format!("{:.1}", stats.wpm), theme.primary),
    (
      "ACCURACY",
      format!("{:.1}%", stats.accuracy * 100.0),
      theme.ink,
    ),
    ("ERRORS", format!("{}", stats.error_count), theme.ink),
    (
      "DURATION",
      format!("{:.1}s", stats.duration_ms / 1000.0),
      theme.ink,
    ),
  ];
  let block = (CARD_WIDTH - PAD * 2.0) / 4.0;
  for (index, (label, value, color)) in entries.iter().enumerate() {
    let x = PAD + index as f64 * block;
    ctx.set_fill_style_str(&rgb(*color));
    ctx.set_font(&format!("600 {STATS_VALUE_SIZE}px {FONT_MONO}"));
    fill_text(ctx, value, x, layout.stats_value_baseline)?;
    ctx.set_fill_style_str(&rgb(theme.body_mid));
    ctx.set_font(&format!("500 {STATS_LABEL_SIZE}px {FONT_DISPLAY}"));
    fill_text(ctx, label, x, layout.stats_label_baseline)?;
  }

  ctx.set_fill_style_str(&rgb(theme.mute));
  ctx.fill_rect(PAD, layout.divider_y, CARD_WIDTH - PAD * 2.0, 1.0);
  Ok(())
}

/// Syntax-colored code excerpt inside the soft panel.
fn draw_excerpt(
  ctx: &CanvasRenderingContext2d,
  excerpt: &[Vec<CodeRun>],
  highlight: &HighlightData,
  layout: &Layout,
  theme: &CardTheme,
) -> Result<(), String> {
  ctx.set_fill_style_str(&rgb(theme.canvas_soft));
  rounded_rect(
    ctx,
    PAD,
    layout.panel_top,
    CARD_WIDTH - PAD * 2.0,
    layout.panel_height,
    12.0,
  )?;
  ctx.fill();

  let mut baseline = layout.panel_top + PANEL_PAD + CODE_SIZE * 0.78;
  for line in excerpt {
    let mut x = PAD + PANEL_PAD;
    for run in line {
      let token = highlight.colors.get(run.class).copied();
      let color = token.map(|t| (t.r, t.g, t.b)).unwrap_or(theme.ink);
      let bold = token.is_some_and(|t| t.bold);
      let italic = token.is_some_and(|t| t.italic);
      let weight = if bold { "700" } else { "400" };
      let slant = if italic { "italic " } else { "" };
      ctx.set_fill_style_str(&rgb(color));
      ctx.set_font(&format!("{slant}{weight} {CODE_SIZE}px {FONT_MONO}"));
      fill_text(ctx, &run.text, x, baseline)?;
      x += ctx.measure_text(&run.text).map_err(js_err)?.width();
    }
    baseline += CODE_LINE_HEIGHT;
  }
  Ok(())
}

/// Take the first `max_lines` lines of `code` as syntax-class runs,
/// expanding tabs to four spaces and truncating long lines with `…`.
fn excerpt_runs(
  code: &str,
  char_classes: &[usize],
  max_lines: usize,
  max_chars: usize,
) -> Vec<Vec<CodeRun>> {
  let mut lines: Vec<Vec<CodeRun>> = Vec::new();
  let mut line: Vec<CodeRun> = Vec::new();
  let mut line_chars = 0usize;
  let mut truncated = false;

  if code.is_empty() || max_lines == 0 {
    return lines;
  }

  for (index, ch) in code.chars().enumerate() {
    if ch == '\n' {
      if truncated {
        push_run(&mut line, 0, "…");
      }
      lines.push(std::mem::take(&mut line));
      if lines.len() == max_lines {
        return lines;
      }
      line_chars = 0;
      truncated = false;
      continue;
    }
    if truncated {
      continue;
    }
    let class = char_classes.get(index).copied().unwrap_or(0);
    let width = if ch == '\t' { 4 } else { 1 };
    if line_chars + width > max_chars {
      truncated = true;
      continue;
    }
    match line.last_mut() {
      Some(run) if run.class == class => {
        if ch == '\t' {
          run.text.push_str("    ");
        } else {
          run.text.push(ch);
        }
      }
      _ => {
        let mut text = String::new();
        if ch == '\t' {
          text.push_str("    ");
        } else {
          text.push(ch);
        }
        line.push(CodeRun { class, text });
      }
    }
    line_chars += width;
  }

  if !line.is_empty() || truncated {
    if truncated {
      push_run(&mut line, 0, "…");
    }
    lines.push(line);
  }
  lines
}

/// Append `text` to the last run when the class matches, else start a run.
fn push_run(line: &mut Vec<CodeRun>, class: usize, text: &str) {
  match line.last_mut() {
    Some(run) if run.class == class => run.text.push_str(text),
    _ => line.push(CodeRun {
      class,
      text: text.to_string(),
    }),
  }
}

/// Compute every vertical position for a card with `excerpt_lines` lines.
fn layout(excerpt_lines: usize) -> Layout {
  let title_baseline = PAD + LOGO + TITLE_GAP;
  let stats_value_baseline = title_baseline + STATS_GAP + STATS_VALUE_SIZE * 0.72;
  let stats_label_baseline = stats_value_baseline + STATS_LABEL_SIZE + 8.0;
  let divider_y = stats_label_baseline + DIVIDER_GAP + 8.0;
  let panel_top = divider_y + PANEL_GAP;
  let panel_height = PANEL_PAD * 2.0 + excerpt_lines as f64 * CODE_LINE_HEIGHT;
  Layout {
    title_baseline,
    stats_value_baseline,
    stats_label_baseline,
    divider_y,
    panel_top,
    panel_height,
    card_height: panel_top + panel_height + PAD,
  }
}

/// Resolve the card colors from the document's design tokens.
fn read_theme() -> CardTheme {
  let defaults = CardTheme::default();
  let Some(style) = root_style() else {
    return defaults;
  };
  CardTheme {
    canvas: token(&style, "--color-canvas").unwrap_or(defaults.canvas),
    canvas_soft: token(&style, "--color-canvas-soft").unwrap_or(defaults.canvas_soft),
    ink: token(&style, "--color-ink").unwrap_or(defaults.ink),
    body_mid: token(&style, "--color-body-mid").unwrap_or(defaults.body_mid),
    mute: token(&style, "--color-mute").unwrap_or(defaults.mute),
    primary: token(&style, "--color-primary").unwrap_or(defaults.primary),
    on_primary: token(&style, "--color-on-primary").unwrap_or(defaults.on_primary),
  }
}

/// Computed style of the document root, where the tokens live.
fn root_style() -> Option<CssStyleDeclaration> {
  let window = web_sys::window()?;
  let root = window.document()?.document_element()?;
  window.get_computed_style(&root).ok()?
}

/// Read one design token and parse it as an RGB color.
fn token(style: &CssStyleDeclaration, name: &str) -> Option<Rgb> {
  parse_hex_color(&style.get_property_value(name).ok()?)
}

/// Parse a `#rgb` or `#rrggbb` color string.
fn parse_hex_color(value: &str) -> Option<Rgb> {
  let hex = value.trim().strip_prefix('#')?;
  if !hex.is_ascii() {
    return None;
  }
  match hex.len() {
    3 => {
      let mut digits = hex.chars();
      let r = hex_digit(digits.next()?) * 17;
      let g = hex_digit(digits.next()?) * 17;
      let b = hex_digit(digits.next()?) * 17;
      Some((r, g, b))
    }
    6 => {
      let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
      let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
      let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
      Some((r, g, b))
    }
    _ => None,
  }
}

/// Hex digit value.
fn hex_digit(digit: char) -> u8 {
  digit.to_digit(16).unwrap_or(0) as u8
}

/// Today's local date as `YYYY-MM-DD`.
fn format_date(now_ms: f64) -> String {
  let date = js_sys::Date::new(&JsValue::from_f64(now_ms));
  format!(
    "{:04}-{:02}-{:02}",
    date.get_full_year(),
    date.get_month() + 1,
    date.get_date()
  )
}

/// Download filename, e.g. `rusttype-hello-world-42wpm.png`.
fn share_filename(title: &str, wpm: f64) -> String {
  format!("rusttype-{}-{:.0}wpm.png", slug(title), wpm)
}

/// Lowercase, dash-separated slug of `title`, falling back to `session`.
fn slug(title: &str) -> String {
  let mut slug = String::new();
  let mut last_dash = true;
  for ch in title.chars() {
    if slug.len() >= MAX_SLUG {
      break;
    }
    if ch.is_ascii_alphanumeric() {
      slug.push(ch.to_ascii_lowercase());
      last_dash = false;
    } else if !last_dash {
      slug.push('-');
      last_dash = true;
    }
  }
  while slug.ends_with('-') {
    slug.pop();
  }
  if slug.is_empty() {
    "session".to_string()
  } else {
    slug
  }
}

/// Trace a rounded-rectangle path.
fn rounded_rect(
  ctx: &CanvasRenderingContext2d,
  x: f64,
  y: f64,
  width: f64,
  height: f64,
  radius: f64,
) -> Result<(), String> {
  let r = radius.min(width / 2.0).min(height / 2.0);
  ctx.begin_path();
  ctx.move_to(x + r, y);
  ctx
    .arc_to(x + width, y, x + width, y + height, r)
    .map_err(js_err)?;
  ctx
    .arc_to(x + width, y + height, x, y + height, r)
    .map_err(js_err)?;
  ctx.arc_to(x, y + height, x, y, r).map_err(js_err)?;
  ctx.arc_to(x, y, x + width, y, r).map_err(js_err)?;
  ctx.close_path();
  Ok(())
}

/// Truncate `text` with `…` until it fits `max_width`.
fn fit_text(ctx: &CanvasRenderingContext2d, text: &str, max_width: f64) -> String {
  let width = |candidate: &str| {
    ctx
      .measure_text(candidate)
      .map(|m| m.width())
      .unwrap_or(0.0)
  };
  if width(text) <= max_width {
    return text.to_string();
  }
  let mut fitted = String::new();
  for ch in text.chars() {
    let mut candidate = fitted.clone();
    candidate.push(ch);
    candidate.push('…');
    if width(&candidate) > max_width {
      break;
    }
    fitted.push(ch);
  }
  fitted.push('…');
  fitted
}

/// `rgb(...)` string for a canvas fill/stroke style.
fn rgb(color: Rgb) -> String {
  format!("rgb({},{},{})", color.0, color.1, color.2)
}

/// Canvas text helper.
fn fill_text(ctx: &CanvasRenderingContext2d, text: &str, x: f64, y: f64) -> Result<(), String> {
  ctx.fill_text(text, x, y).map_err(js_err)
}

/// Wrap `canvas.toBlob` (callback-based) in a future.
async fn canvas_to_blob(canvas: &HtmlCanvasElement) -> Result<Blob, String> {
  let promise = Promise::new(&mut |resolve: Function, _reject: Function| {
    let resolve_for_callback = resolve.clone();
    let callback = Closure::once(move |blob: Option<Blob>| {
      let value: JsValue = match blob {
        Some(blob) => blob.into(),
        None => JsValue::NULL,
      };
      let _ = resolve_for_callback.call1(&JsValue::UNDEFINED, &value);
    });
    if canvas
      .to_blob_with_type(callback.as_ref().unchecked_ref(), "image/png")
      .is_err()
    {
      // Settle the promise even if the call itself threw.
      let _ = resolve.call1(&JsValue::UNDEFINED, &JsValue::NULL);
    }
    // The browser holds the callback until it fires; leak is one-shot.
    callback.forget();
  });
  let value = JsFuture::from(promise).await.map_err(js_err)?;
  value
    .dyn_into::<Blob>()
    .map_err(|_| "canvas.toBlob returned no data".to_string())
}

/// Resolve after `ms` milliseconds.
async fn wait_ms(ms: i32) {
  let promise = Promise::new(&mut |resolve: Function, _reject: Function| {
    let callback = Closure::once(move || {
      let _ = resolve.call0(&JsValue::UNDEFINED);
    });
    if let Some(window) = web_sys::window() {
      let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        callback.as_ref().unchecked_ref(),
        ms,
      );
    }
    callback.forget();
  });
  let _ = JsFuture::from(promise).await;
}

/// Stringify a JS error for the UI.
fn js_err(error: JsValue) -> String {
  format!("{error:?}")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_hex_color_reads_shorthand_and_long_form() {
    assert_eq!(parse_hex_color("#fff"), Some((255, 255, 255)));
    assert_eq!(parse_hex_color("#ff4f00"), Some((255, 79, 0)));
    assert_eq!(parse_hex_color(" #ff4f00 "), Some((255, 79, 0)));
    assert_eq!(parse_hex_color("#12345"), None);
    assert_eq!(parse_hex_color("red"), None);
  }

  #[test]
  fn slug_normalizes_titles() {
    assert_eq!(slug("Hello World!"), "hello-world");
    assert_eq!(slug("!!!"), "session");
    assert_eq!(slug("impl fmt::Display"), "impl-fmt-display");
  }

  #[test]
  fn share_filename_contains_slug_and_wpm() {
    assert_eq!(
      share_filename("Hello World!", 42.4),
      "rusttype-hello-world-42wpm.png"
    );
  }

  #[test]
  fn excerpt_limits_lines() {
    let code = "a\nb\nc\nd";
    let classes = vec![0; 7];
    let lines = excerpt_runs(code, &classes, 2, 80);
    assert_eq!(lines.len(), 2);
  }

  #[test]
  fn excerpt_keeps_blank_lines() {
    let code = "a\n\nb";
    let classes = vec![0; 4];
    let lines = excerpt_runs(code, &classes, 8, 80);
    assert_eq!(lines.len(), 3);
    assert!(lines[1].is_empty());
  }

  #[test]
  fn excerpt_expands_tabs_and_groups_runs() {
    let code = "fn main() {\n\tlet x = 1;\n}";
    let classes = vec![1; code.chars().count()];
    let lines = excerpt_runs(code, &classes, 8, 80);
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[1][0].text, "    let x = 1;");
    assert_eq!(lines[1][0].class, 1);
  }

  #[test]
  fn excerpt_splits_runs_by_class() {
    let lines = excerpt_runs("ab", &[1, 2], 4, 80);
    assert_eq!(
      lines[0],
      vec![
        CodeRun {
          class: 1,
          text: "a".into()
        },
        CodeRun {
          class: 2,
          text: "b".into()
        },
      ]
    );
  }

  #[test]
  fn excerpt_truncates_long_lines_with_ellipsis() {
    let lines = excerpt_runs("abcdef", &[0; 6], 4, 3);
    assert_eq!(lines[0].last().expect("run").text, "abc…");
  }

  #[test]
  fn layout_grows_with_excerpt_lines() {
    assert_eq!(layout(0).panel_height, PANEL_PAD * 2.0);
    assert!(layout(8).card_height > layout(0).card_height);
  }
}
