import * as wasm from "wasm-audio";

const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

navigator.getUserMedia(
  { audio: true },
  stream => {
    const src = audioCtx.createMediaStreamSource(stream);
    src.connect(audioCtx.destination);
  },
  err => console.error(err)
);

wasm.greet();
