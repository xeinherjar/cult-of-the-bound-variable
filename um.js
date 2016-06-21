var fileEl = document.getElementById('file');
var consoleEl = document.getElementById('console');

var file = null;
fileEl.addEventListener('change', function(e) {
  var f = e.target.files[0];
  var reader = new FileReader();

  reader.onloadend = (function(file) {
    var file = new Uint8Array(reader.result);
  });
  
  reader.readAsArrayBuffer(f);
}, false);

var um = (function() {
  // Program Counter | Execution Finger
  var pc = 0;

  // 8 Registers
  var registers = [0, 0, 0, 0, 0, 0, 0, 0];
  // Register Index (Taken from op)
  var regA, regB, regC;

  // Memory
  var mem = [];

  // Standard Ops

  // op 0
  function conditionalMove() {
    if (registers[regC]) {
      registers[regA] = registers[regB];
    }
  };

  // op 1
  function arrayIndex() {
    var idx = registers[regB];
    var offset = registers[regC];
    registers[regA] = mem[idx][offset];
  };

  var powerOn = function powerOn(file) {

  };

  var step = function step() {
    console.log('step');
  };

  return {
    powerOn: powerOn,
    step: step,
  };
})();

um.step();
