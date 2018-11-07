#[auto_enum(u32, checked)]
/// Specifies the direction in which reading progresses.
///
/// <div style="padding: 10px 10px 2px 10px; margin: 10px; background-color: #F2F2F2">
///
/// **Note**
/// `TopToBottom` and `BottomToTop` are available in Windows 8.1 and later only.
///
/// </div>
pub enum ReadingDirection {
    /// Indicates that reading progresses from left to right.
    LeftToRight = 0,

    /// Indicates that reading progresses from right to left.
    RightToLeft = 1,

    /// Indicates that reading progresses from top to bottom.
    TopToBottom = 2,

    /// Indicates that reading progresses from bottom to top.
    BottomToTop = 3,
}
