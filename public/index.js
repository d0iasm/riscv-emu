import init, { Emulator } from "./pkg/rvemu_wasm.js";

const fileIn = document.getElementById("file");
// The `buffer`, that will be observed for mutations, stores the output from Rust.
const buffer = document.getElementById("buffer");
// The `buffer8`, that will be observed for mutations, stores the 1 byte from
// Rust.
const buffer8 = document.getElementById("buffer8");
// Options for the observer (which mutations to observe)
const config = { childList: true, subtree: true };

const termContainer = document.getElementById("terminal");
const term  = new Terminal({cursorBlink: true});

const fitAddon = new FitAddon.FitAddon();
const newLine = "\r\n$ ";
const deleteLine = "\x1b[2K\r";

let emu = null;

const fileReader = new FileReader();
let files = [];

// Callback function to execute when mutations are observed.
const callback = function(mutationsList, observer) {
  for(let mutation of mutationsList) {
    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
      const firstChild = mutation.addedNodes[0];
      const texts = firstChild.innerText.split("\n");
      for (let i=0; i<texts.length; i++) {
        term.writeln(texts[i]);
      }
      buffer.removeChild(firstChild);
      term.write("$ ");
    }
  }
};

let buffer_count = 0;

// Callback function to execute when mutations are observed.
const callback8 = function(mutationsList, observer) {
  for(let mutation of mutationsList) {
    if (buffer_count == 0 && mutation.addedNodes.length > 0) {
      term.write(deleteLine);
    }

    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
      buffer_count++;
      const firstChild = mutation.addedNodes[0];
      let c = firstChild.innerText;
      buffer8.removeChild(firstChild);
      if (c != "\n") {
        term.write(c);
      } else {
        term.writeln("");
      }
    }

    if (mutation.type === 'childList' && mutation.removedNodes.length > 0) {
      buffer_count--;
    }

    if (buffer_count == 0 && mutation.removedNodes.length > 0) {
      term.write("$ ");
    }
  }
};

// Create an observer instance linked to the callback function.
const observer = new MutationObserver(callback);
const observer8 = new MutationObserver(callback8);

async function initialize() {
  // Load the wasm file.
  await init();

  term.loadAddon(fitAddon);
  term.open(termContainer);
  fitAddon.fit();

  runTerminal();

  // Start observing the target node for configured mutations
  observer8.observe(buffer8, config);
  observer.observe(buffer, config);

  fileReader.onloadend = e => {
    emu = Emulator.new();
    const bin = new Uint8Array(fileReader.result);
    emu.set_dram(bin);
    try {
      emu.start();
    } catch(err) {
      term.write(deleteLine);
      term.write(err.message);
      prompt();
      console.log(err);
    } finally {
      emu.dump_registers();
      emu = null;
    }
  };

  fileIn.onchange = e => {
    let names = "";
    for (let i=0; i<file.files.length; i++) {
      names += file.files[i].name;
      if (i < file.files.length-1) {
        names += "\t";
      }
    }
    files.push.apply(files, file.files);
    term.write(deleteLine);
    term.write("uploaded:\t" + names);
    prompt();
  };
}

function prompt() {
  term.write(newLine);
}

function help() {
  term.writeln("Supports the following commands:");
  term.writeln("  upload      open local files for the execution on the emulator");
  term.writeln("  ls          list files you uploaded");
  term.writeln("  run [file]  execute a file");
  term.write("  help        print all commands you can use");
}

function upload() {
  fileIn.click();
}

function run(filename) {
  for (let i=0; i<files.length; i++) {
    if (filename == files[i].name) {
      fileReader.readAsArrayBuffer(files[i]);
      return;
    }
  }
  term.write("\r\n" + filename + ": No such file");
}

function ls() {
  let names = "";
  for (let i=0; i<files.length; i++) {
    names += files[i].name;
    if (i < files.length-1) {
      names += "\t";
    }
  }
  term.write("\r\n" + names);
}

function command(input) {
  const com = input.split(" ").filter(element => element.length != 0);
  switch (com[0]) {
    case "":
      break;
    case "upload":
      upload();
      break;
    case "ls":
      ls();
      break;
    case "run":
      if (com[1] == undefined) {
        term.writeln("");
        help();
        break;
      }
      run(com[1]);
      break;
    default:
      term.writeln("");
      help();
  }
}

function loadApps() {
  fetch("./apps/fib.text")
    .then(response => response.blob())
    .then(blob => {
      const sampleFile = new File([blob], "fib");
      files.push(sampleFile);
    });

  fetch("./apps/echoback.text")
    .then(response => response.blob())
    .then(blob => {
      const sampleFile = new File([blob], "echoback");
      files.push(sampleFile);
    });

  fetch("./apps/xv6.text")
    .then(response => response.blob())
    .then(blob => {
      const sampleFile = new File([blob], "xv6");
      files.push(sampleFile);
    });
}

function runTerminal() {
  if (term._initialized) {
      return;
  }

  term._initialized = true;

  term.prompt = () => {
    term.write('\r\n$ ');
  };

  term.writeln("Welcome to RISC-V emulator!");
  term.writeln("This is a work-in-progress project. You can see the progress at https://github.com/d0iasm/rvemu");
  term.writeln("Bug reports and feature requests are always welcome: https://github.com/d0iasm/rvemu/issues");
  term.writeln("");
  help();
  term.writeln("");
  prompt();

  loadApps();

  let input = "";
  let cursor = 0;
  term.onKey(e => {
    const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey && !e.domEvent.ctrlKey && !e.domEvent.metaKey;

    if (e.domEvent.code == 'Enter') {
      command(input);
      input = "";
      cursor = 0;
      prompt();
    } else if (e.domEvent.code == 'Backspace') {
      // Do not delete the prompt
      if (term._core.buffer.x > 2) {
        input = input.substr(0, cursor-1) + input.substr(cursor);
        if (cursor > input.length) {
          cursor--;
        }
        term.write(deleteLine);
        term.write("$ ");
        term.write(input);
      }
    } else if (e.domEvent.code == 'ArrowLeft') {
      if (cursor > 0) {
        cursor--;
      }
      term.write(e.key);
    } else if (e.domEvent.code == 'ArrowRight') {
      if (cursor < input.length) {
        cursor++;
      }
      term.write(e.key);
    } else if (printable) {
      cursor++;
      input = input.substr(0, cursor) + e.key + input.substr(cursor);
      term.write(e.key);
    }
  });
}

initialize();
