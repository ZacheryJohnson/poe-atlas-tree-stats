import * as wasm from "wasm-poe-atlas";

//wasm.poe_parse('');

const textInput = document.getElementById('b64_str');
const button = document.getElementById('submit_button');
const textOutput = document.getElementById('stats');

button.onclick = () => {
  const text = /[^/]*$/.exec(textInput.value)[0];
  const parsed = wasm.poe_parse(text);
  textOutput.innerText = parsed;
};