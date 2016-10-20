var fileEl = document.getElementById('file');
var consoleEl = document.getElementById('console');

var file = null;
fileEl.addEventListener('change', function(e) {
  var f = e.target.files[0];
  var reader = new FileReader();

  reader.onloadend = (function(file) {
    var file = new Uint8Array(reader.result);
    um.powerOn(file);
  });

  reader.readAsArrayBuffer(f);
}, false);

var um = (function() {
  var power = false;
  var animationFrame = null;
  var waitForInput = false;

  // Program Counter | Execution Finger
  var pc = 0;

  // 8 Registers
  var registerBuffer = new ArrayBuffer(8 * 4);
  var registers = new Uint32Array(registerBuffer);

  // Memory
  var mem = [];
  var mallocIdx = 0;
  var mallocArr = [];

  var powerOn = function powerOn(file) {
    var buffer = new ArrayBuffer(file.length);
    var malloc = new Uint32Array(buffer);

    for (var i = 0, idx = 0; i < file.length; i = i + 4, idx++) {
      var b1 = file[i + 0] << 24;
      var b2 = file[i + 1] << 16;
      var b3 = file[i + 2] <<  8;
      var b4 = file[i + 3];
      malloc[idx] = b1 | b2 | b3 | b4;
    }

    console.log('Start:', new Date());
    mem.push(malloc);
    power = true;
    //loop();
    looper();
  };

  var loop = function() {
    step();
    if (!power) { return; }
    animationFrame = requestAnimationFrame(loop);
  };

  var looper = function() {
    animationFrame = setInterval(main, 0);
  };

  var main = function() {
    console.log('main!');
    var x = 0;
    while(x < 10000) {
      step();
      x++
    };
  };

  var powerOff = function() {
    console.log('power off');
    power = false;
    clearInterval(animationFrame);
  };

  var step = function step() {
    var op = mem[0][pc];
    var a = (op >>> 6) & 0x7;
    var b = (op >>> 3) & 0x7;
    var c =  op        & 0x7;

    //console.log(op >>> 28, a, b, c);

    pc += 1;

    switch(op >>> 28) {
      // Standard Ops
      case 0: // conditional move
        if (registers[c] !== 0) {
          registers[a] = registers[b];
        }
        break;

      case 1: // array index
        var idx = registers[b];
        var offset = registers[c];
        registers[a] = mem[idx][offset];
        break;

      case 2: // array amendment
        var idx = registers[a];
        var offset = registers[b];
        mem[idx][offset] = registers[c];
        break;

      case 3: // addition
        var value = registers[b] + registers[c];
        registers[a] = value;
        break;

      case 4: // multiplication
        var value = registers[b] * registers[c];
        registers[a] = value;
        break;

      case 5: // division
        var value = registers[b] / registers[c];
        registers[a] = value;
        break;

      case 6: // not and
        var value = ~(registers[b] & registers[c]);
        registers[a] = value;
        break;

      // Other ops
      case 7: // halt
        console.log('HALT', new Date());
        power = false;
        break;

      case 8: // allocation
        var buffer = new ArrayBuffer(4 * registers[c]);
        var malloc = new Uint32Array(buffer);
        if (mallocArr.length) {
          var idx = mallocArr.pop();
          mem[idx] = malloc;
          registers[b] = idx;
        } else {
          mallocIdx += 1;
          //mem[mallocIdx] = malloc;
          mem.push(malloc);
          registers[b] = mallocIdx;
        }
        break;

      case 9: // abandonment
        var idx = registers[c];
        mem[idx] = [];
        mallocArr.push(idx);
        break;

      case 10: // output
        var value = String.fromCharCode(registers[c] & 0xFF);
        consoleEl.textContent += value;
        break;

      case 11: // input
        console.log('input...');
        break;

      case 12: // load program
        var idx = registers[b];
        var program = Uint32Array.from(mem[idx]);
        mem[0] = program;
        pc = registers[c];
        break;

      case 13:
        a = (op >>> 25) & 0x7;
        var value = op & 0x01FFFFFF;
        registers[a] = value;
        break;

      default:
        power = false;
        console.log('unknown', op);
        return false

      }

    };

    var getPower = function() {
      return power;
  };


  return {
    powerOn: powerOn,
    step: step,
    getPower: getPower,
    loop: loop,
    powerOff: powerOff
  };
})();

