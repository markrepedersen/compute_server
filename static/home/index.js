document.addEventListener('dragover', function(e) {
    const dropZone = document.getElementById("dropZone")
    let found = false;
    let node = e.target;
    
    do {
	if (node === dropZone[0]) {
	    found = true;
	    break;
	}
	node = node.parentNode;
    } while (node != null);

    if (found) {
	dropZone.addClass('hover');
    } else {
	dropZone.removeClass('hover');
    }

    window.dropZoneTimeout = setTimeout(function() {
	window.dropZoneTimeout = null;
	dropZone.removeClass('in hover');
    }, 100);
});
