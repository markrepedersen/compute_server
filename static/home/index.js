const dropZone = document.getElementById("dropZone");
const fileInput = document.getElementById("files");
const IMPORT_ROUTE = "/import";

dropZone.addEventListener("drop", dropHandler)
dropZone.addEventListener("dragover", function (e) {
    e.preventDefault();
})

fileInput.addEventListener("change", function(e) {
    sendFiles(fileInput.files)
})

function dropHandler(event) {
    const files = [];
    
    event.preventDefault();

    if (event.dataTransfer.items) {
	for (var i = 0; i < event.dataTransfer.items.length; i++) {
	    if (event.dataTransfer.items[i].kind === 'file') {
		files.push(event.dataTransfer.items[i].getAsFile());
	    }
	}
    } else {
	for (var i = 0; i < event.dataTransfer.files.length; i++) {
	    files.push(event.dataTransfer.files[i]);
	}
    }

    sendFiles(files);
}

// Sends the selected files to the server.
async function sendFiles(files) {
    let formData = new FormData();

    for (const file of files) {
	console.log(file.name);
	formData.append('files[]', file, file.name);	
    }

    const response = await fetch(IMPORT_ROUTE, {
	method: "POST",
	body: formData
    });
}
