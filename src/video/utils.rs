use std::fs::File;
use serde::{
    Serialize,
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};


#[derive(Deserialize, Serialize,Default)]
pub struct UploadingVideos{
    pub main_video:Option<File>,
     pub video_caption:Option<File>,
     pub video_thumbnail:Option<File>,

}

impl UploadingVideos {
    pub fn new(main_video: Option<File>, video_caption: Option<File>, video_thumbnail: Option<File>) -> Self {
        UploadingVideos { main_video, video_caption, video_thumbnail }
    }
    pub fn set_main_video(&mut self, main_video: Option<File>) {
        self.main_video = main_video;
    }
    pub fn set_video_caption(&mut self, video_caption: Option<File>) {
        self.video_caption = video_caption;
    }
    pub fn set_video_thumbnail(&mut self, video_thumbnail: Option<File>) {
        self.video_thumbnail = video_thumbnail;
    }
}


#[derive(Deserialize, Serialize,Clone,Default)]
pub struct InitVideoParams{
    owner:String,
     purpose: String,
     fileSizeBytes: u64,
     uploadCaptions: bool,
     uploadThumbnail: bool
}

impl InitVideoParams {
    pub fn new(purpose: String, fileSizeBytes: u64, uploadCaptions: bool, uploadThumbnail: bool, owner:String) -> Self {
        InitVideoParams { purpose, fileSizeBytes, uploadCaptions, uploadThumbnail,owner }
    }

    pub fn set_owner(&mut self, owner: String) {
        self.owner = owner;
    }
    pub fn set_purpose(&mut self, purpose: String) {
        self.purpose = purpose;
    }
    pub fn set_fileSizeBytes(&mut self, fileSizeBytes: u64) {
        self.fileSizeBytes = fileSizeBytes;
    }
    pub fn set_uploadCaptions(&mut self, uploadCaptions: bool) {
        self.uploadCaptions = uploadCaptions;
    }
    pub fn set_uploadThumbnail(&mut self, uploadThumbnail: bool) {
        self.uploadThumbnail = uploadThumbnail;
    }
}


#[derive(Deserialize, Serialize,Clone,Default)]
pub struct InitVideoResponse{
 pub value:Values
}

#[derive(Deserialize, Serialize,Default)]
pub struct Values {
    pub uploadUrlsExpireAt: u64,
    pub video:String,
    pub uploadInstructions:UploadInstructions,
    pub uploadToken:String,
   pub  captionsUploadUrl:String,
    pub thumbnailUploadUrl:String

}

#[derive(Deserialize, Serialize,Default)]
pub struct  UploadInstructions{
    pub lastByte:u64,
    pub firstByte:u64,
   pub  uploadUrl:String,
}
