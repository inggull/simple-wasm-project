let a = parseInt(prompt("a = "));
let b = parseInt(prompt("b = "));
fetch('target/wasm32-unknown-unknown/debug/add.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)
).then(results => {
    let add = results.instance.exports.add;
    let c = add(a, b);
    console.log(c);
    let html = "<p>" + a + " + " + b + " = " + c + "</p>";
    document.body.innerHTML += html;
});
console.log("Done");