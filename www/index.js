import * as wasm from "wasm-poe-atlas";

//wasm.poe_parse('');

const textInput = document.getElementById('b64_str');
const button = document.getElementById('submit_button');
const statList = document.getElementById('stats');
const shouldCollapse = document.getElementById('collapse');

button.onclick = () => {
  const text = /[^/]*$/.exec(textInput.value)[0];
  console.log(shouldCollapse.checked);
  const parsed = wasm.poe_parse(text, shouldCollapse.checked);
  
  statList.innerHTML = '';
  for (const stat of parsed.split('\n')) {
    if (stat.trim().length == 0) {
      continue;
    }
    const li = document.createElement('li');

    li.innerHTML = stat;

    statList.appendChild(li);
  }
};