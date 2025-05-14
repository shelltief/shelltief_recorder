
function is_macos {
	if ! test "$(uname)" = "Darwin";
	then echo "This script runs only on macos"; exit 1;
	fi
}

function get_external_disk {
	declare external_disk;

	external_disk="$(diskutil list | grep external | cut -f 1 -d ' ' | head -n 1)"
	if test -z "$external_disk";
	then echo "No external disk found";
	else echo "External disk is : '$external_disk'";
	fi
}

function main {
	is_macos;
	get_external_disk;
}

main "$@";
