/// `XkcdResponse` is the outer wrapper for all results from the XKCD API query.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct XkcdResponse {
    /// The month the comic was published in, represented as an integer from 1
    /// to 12.
    pub month: u8,
    /// The number/ID of the comic.
    pub num: u32,
    /// The URL in the anchor tag of the hyperlink surrounding the image, if the
    /// image is a hyperlinked one.
    pub link: String,
    /// The year the comic was published in.
    pub year: u32,
    /// News or updates regarding the comic.
    pub news: String,
    /// A plain ASCII representation of the title.
    pub safe_title: String,
    /// A transcript of the text of the comic.
    pub transcript: String,
    /// Alt text for the comic.
    pub alt: String,
    /// A link to the comic image.
    pub img: Url,
    /// The title of the comic.
    pub title: String,
    /// The day of the month the comic was published on.
    pub day: u8,
}
