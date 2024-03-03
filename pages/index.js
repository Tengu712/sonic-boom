const SAMPLE_RATE = 44100
const SIZE_OF_32BITS = 4

function getUint32ArrayAt(wasmInstance, pointer) {
  const array = new Uint32Array(wasmInstance.exports.memory.buffer, pointer, 1)
  return array[0]
}

function getFloat32ArrayBetween(wasmInstance, pointer, length) {
  return new Float32Array(wasmInstance.exports.memory.buffer, pointer, length)
}

function createAudioBuffer(audioContext, waveBuffers) {
  const maxWaveBufferSize = waveBuffers.reduce((a, b) => Math.max(a.length, b.length)).length
  const audioBuffer = audioContext.createBuffer(waveBuffers.length, maxWaveBufferSize, SAMPLE_RATE)
  for (let i = 0; i < waveBuffers.length; ++i) {
    audioBuffer.copyToChannel(waveBuffers[i], i)
  }
  return audioBuffer
}

function playAudioBuffer(audioContext, audioBuffer) {
  const source = audioContext.createBufferSource()
  source.buffer = audioBuffer
  source.connect(audioContext.destination)
  source.start()
}

document.addEventListener("DOMContentLoaded", async () => {
  // get elements
  const btnPlay = document.getElementById("play")
  const taMML = document.getElementById("mml")

  // create a WASM instance
  const wasmInstance = await fetch("./iam-mml.wasm")
    .then((response) => response.arrayBuffer())
    .then((bytes) => WebAssembly.instantiate(bytes))
    .then((wasm) => wasm.instance)

  // add the event listener when clicking the play button
  btnPlay.addEventListener("click", () => {
    const resultPointer = wasmInstance.exports.create_wave()
    const waveBufferPointer = getUint32ArrayAt(wasmInstance, resultPointer)
    const waveBufferLength = getUint32ArrayAt(wasmInstance, resultPointer + SIZE_OF_32BITS)
    const waveBuffer = getFloat32ArrayBetween(wasmInstance, waveBufferPointer, waveBufferLength)
    const audioContext = new AudioContext()
    const audioBuffer = createAudioBuffer(audioContext, [waveBuffer])
    playAudioBuffer(audioContext, audioBuffer)
  })
})
