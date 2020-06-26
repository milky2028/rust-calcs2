const { total_value } = require('./pkg')
let items = [{ id: 'first', value: 30 }, { id: 'first', value: 30 }, { id: 'first', value: 30 }];
const res = total_value(items);

console.log(res);