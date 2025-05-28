async function search(prompt) {
    const result = document.getElementById("results");
    result.innerHTML = "";

    const response = await fetch("/api/search", {
        method: "POST",
        headers: {"Content-Type": "text/plain"},
        body: prompt,
    });

    const json = await response.json();

    result.innerHTML = "";

    for ([path, rank] of json) {
        let item = document.createElement("span");
        item.appendChild(document.createTextNode(path));
        item.appendChild(document.createElement("br"));
        result.appendChild(item);
    }
}

let query = document.getElementById("query");
let currentSearch = Promise.resolve();

query.addEventListener("keypress", (e) => {
    if (e.key == "Enter") {
        currentSearch.then(() => search(query.value));
    }
});