const fs = require('fs');
const list = [];
for(let i = 0; i < 100; i++) {
	list.push(Array.from({length: 100 + i * 100}, () => Math.random() * (i + 1)));
}

const content = JSON.stringify(list);
fs.writeFile('./data.json', content, err => {
	if (err) {
		console.error(err);
	}
});