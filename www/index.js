const videoSrc = "http://127.0.0.1:3000/api/hls/playlist/some_stream";

/** @type {HTMLVideoElement} */
const video = document.getElementById("video");

if (Hls.isSupported()) {
  const hls = new Hls({ debug: true });
  hls.loadSource(videoSrc);
  hls.attachMedia(video);
} else if (video.canPlayType("application/vnd.apple.mpegurl")) {
  video.src = videoSrc;
}

function playPauseMedia() {
  if (video.paused) {
    video.play();
    // Skip to the last available fragment (live stream)
    video.currentTime = video.duration;
  } else {
    video.pause();
  }
}

document.querySelector(".play").addEventListener("click", playPauseMedia);
