console.log("after module load");
// const module = import(`./pkg/sudoku_web.js`);
// let solver;

// function loadWebAssembly(fileName) {
//   return fetch(fileName)
//     .then(response => response.arrayBuffer())
//     .then(buffer => WebAssembly.compile(buffer))
//     .then(module => {return new WebAssembly.Instance(module) });
// };

// loadWebAssembly('./pkg/sudoku_web_bg.wasm')
//   .then(instance => {
//     solver = instance.exports.solver;
//     console.log('Finished compiling! Ready when you are...');
//   }); 

const { solver } = wasm_bindgen;

async function run() {
	await wasm_bindgen('./pkg/sudoku_web_bg.wasm');

	const result = solver("1.......2.9.4...5...6...7...5.9.3.......7.......85..4.7.....6...3...9.8...2.....1");
	console.log(`solution: ${result}`);
	if (result !== "174385962293467158586192734451923876928674315367851249719548623635219487842736591") {
		throw new Error("wasm addition doesn't work!");
	}
}

run();

function solve() {
	// solve the puzzles in the textarea
	var s = document.getElementById('text').value.split("\n")
	var v = '', time_beg = new Date().getTime(), cnt = 0;
	for (var i = 0; i < s.length; ++i) {
		if (s[i].length >= 81) {
			var x = solver(s[i])
			v += x + "\n" // output the first solution only
			++cnt;
		}
	}
	document.getElementById('text').value = v
	var t = (new Date().getTime() - time_beg) / 1000.0;
	var t2 = Math.floor(t / cnt * 10000) / 10000
	document.getElementById('runtime').innerHTML = "Solving " + cnt + " Sudokus in " + t + " seconds (" + t2 + " sec / Sudoku)";
	// solve the puzzle in the grid
	var n_hints = 0;
	s = ''
	for (var i = 0; i < 81; ++i) { // get the sudoku string
		var y = document.getElementById('C' + i).value
		if (y >= 1 && y <= 9) {
			s += '' + y;
			++n_hints;
		} else s += '.'
	}
	if (n_hints >= 15) { // enough hints
		var x = solver(s)
		if (x.length == 0) {
			document.getElementById('9x9info').innerHTML = 'No solution'
		} else {
			for (var i = 0; i < 81; ++i)
				document.getElementById('C' + i).value = x[0][i]
			if (x.length == 1) document.getElementById('9x9info').innerHTML = 'Unique solution'
			else document.getElementById('9x9info').innerHTML = 'Multiple solutions'
		}
	} else document.getElementById('9x9info').innerHTML = 'No less than 15 hints are required'
}

function strrep(a, t) { // repeat a string "a" for "t" times
	var s = ''
	for (var i = 0; i < t; ++i) s += a
	return s
}

function set_9x9(str) { // set the grid given a sudoku string
	if (str != null && str.length >= 81) {
		document.getElementById('9x9info').innerHTML = 'Input or select in the textarea to fill the grid'
		for (var i = 0; i < 81; ++i) document.getElementById('C' + i).value = ''
		for (var i = 0; i < 81; ++i)
			if (str.substr(i, 1) >= 1 && str.substr(i, 1) <= 9)
				document.getElementById('C' + i).value = str.substr(i, 1)
	}
}

function draw_9x9() { // generate the grid and fill it (called "onLoad")
	// generate the grid
	var s = '<table class="table">\n'
	for (var i = 0; i < 9; ++i) {
		s += '<tr>'
		for (var j = 0; j < 9; ++j) {
			var c = 'cell'
			if ((i + 1) % 3 == 0 && j % 3 == 0) c = 'cell3'
			else if ((i + 1) % 3 == 0) c = 'cell1'
			else if (j % 3 == 0) c = 'cell2'
			s += '<td class="' + c + '"><input class="input" type="text" size="1" maxlength="1" id="C' + (i * 9 + j) + '"></td>';
		}
		s += '</tr>\n'
	}
	s += '</table>'
	document.getElementById('9x9').innerHTML = s
	// fill the grid if the puzzle is given in the URL
	var inp = document.URL
	var set = false
	if (inp.indexOf('?') >= 0) {
		var match = /[?&]q=([^\s&]+)/.exec(inp)
		if (match.length == 2 && match[1].length >= 81) {
			document.getElementById('text').value = match[1]
			set_9x9(match[1])
			set = true
		}
	}
	// if the grid is empty, set the grid with "Easter Monster"
	if (!set) {
		document.getElementById('text').value = '1.......2.9.4...5...6...7...5.9.3.......7.......85..4.7.....6...3...9.8...2.....1'
		set_9x9(document.getElementById('text').value)
	}
}

function clear_input() {
	document.getElementById('text').value = ''
	document.getElementById('9x9info').innerHTML = 'Input or select in the rextarea to fill the grid'
	for (var i = 0; i < 81; ++i)
		document.getElementById('C' + i).value = ''
}