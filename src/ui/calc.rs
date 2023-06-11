//! A util module that does calculations for UI

/// Returns scrollbar "thumb" height, thumb position respectively in a tuple.
///
/// params:
/// - `disp_content_len`: Displayed content length or the height of the
///     window or box in which content is being displayed.
///
/// - `disp_content_offset`: The offset length of the last line of the content
///     being displayed. Or, "How far is the content being displayed is from
///     the start/top?"
///
/// - `total_content_len`: Length of the total content.
pub fn scrollbar_pos(
    disp_content_len: u16,
    disp_content_offset: u16,
    total_content_len: u16
) -> (u16, u16) {
    let scrollbar_thumb_height: f32 =
        (disp_content_len as f32 / total_content_len as f32) * disp_content_len as f32;

    let scrollbar_thumb_pos: f32 =
        (
            (disp_content_offset as f32 / total_content_len as f32)
            * disp_content_len as f32
        ) - scrollbar_thumb_height as f32;

    (scrollbar_thumb_height.ceil() as u16, scrollbar_thumb_pos.floor() as u16)
}

