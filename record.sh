#!/usr/bin/env bash

RECORDINGS_PATH="/Volumes/T7/code_videos/Rushes"
SCREEN_DISPLAY=Capture


:<<-"COMMENT"
This function was to archive the `current` directory
in binary directories. It is now here as legacy code
and isn't used anymore
COMMENT
function archive_current_binary {
	declare field_width=8;
	declare last;
	last="$(ls -1 "${PROJECT_PATH:?}" | grep -E '[0-1]' | tail -n 1)";
	last="$(echo $last | bc --ibase=2)";
	last="$((last + 1))";
	last="$(echo $last | bc --obase=2)"
	if test "${#last}" -gt "${field_width}";
	then error_exit "wrapped"; fi
	last="$(printf "%0${field_width}d\n" "$last")";
	if test -f "${CURRENT:?}/pids";
	then error_exit "programm still running";  fi
	mv -i "${CURRENT}" "${PROJECT_PATH:?}/${last}"
}

:<<-"ARCHIVE_CURRENT"
This function takes the directory <project_path>/current
and archives it to a numbered directory automatically. It
computes the last numbered directory and increases the 
count automatically
ARCHIVE_CURRENT
function archive_current {
	declare last;
	last="$(ls -1 "${PROJECT_PATH:?}" | grep -E '[0-9]' | sort -h | tail -n 1)";
	last="$((last + 1))";
	if test -f "${CURRENT:?}/pids";
	then error_exit "programm still running";  fi
	mv -i "${CURRENT}" "${PROJECT_PATH:?}/${last}"
}

function error_exit {
	echo "$@" >&2;
	exit 1;
}

:<<-"COMMENT"
Not called
This is a reminder about how to list the devices 
for the ffmpeg avfoundation backend
COMMENT
function list_devices {
	ffmpeg -f avfoundation -list_devices true -i ""
}

:<<-"COMMENT"
Launch the screen and face recording in background
Stores the pid in a file so they can be killed later on
COMMENT
function start_recording {
	if ! ffmpeg -f avfoundation -audio_device_index 1 -i "${SCREEN_DISPLAY}" "${CURRENT}/screen.mp4" &
	then exit 1; fi
	echo $! > "${PIDS_FILE}"
	if ! ffmpeg -f avfoundation -framerate 30 -i "${CAM}" "${CURRENT}/face.mp4" &
	then read p < "${PIDS_FILE}" ; kill $p; exit 1; fi
	echo $! >> "${PIDS_FILE}"
}

:<<-"COMMENT"
Checks if there are more video displays than only the camera and the screen
For now, it only try to grep my iPhone's name (Nokia de Thibault) and returns
the result of the operation
COMMENT
function phone_cam_available {
	if ffmpeg -f avfoundation -list_devices true -i "" 2>&1 | grep Nokia >/dev/null;
	then return 0; else return 1; fi
}

function stop_recording {
	if ! test -f "${PIDS_FILE}"; then echo "file: '${PIDS_FILE}' not found"; exit 1; fi
	while read p; do kill "$p"; done < "${PIDS_FILE}"
	rm "${PIDS_FILE}";
}

:<<-"COMMENT"
To be used to check if the directory <project_path>/current is 
empty
COMMENT
function is_empty {
	if test "$(ls "$1" | wc -l)" -eq "0";
	then return 0; else return 1; fi
}

:<<-"COMMENT"
To be used when the user needs to choose between two options
COMMENT
function user_continues {
	read -p "${1} [y/n]" -n 1;
	case "$REPLY" in 
		[Y/y]) return 0;;
		*) return 1;;
	esac
}

:<<-"COMMENT"
Checks for existence of project path, number of cameras 
and sets up variable for recording
COMMENT
function setup {
	CAM=FaceTime
	PROJECT_NAME="$1"
	PROJECT_PATH="${RECORDINGS_PATH}/${PROJECT_NAME}"
	CURRENT="${PROJECT_PATH}/current"
	PIDS_FILE="${CURRENT}/pids"
	if ! test -d "${PROJECT_PATH}";
	then error_exit "'${PROJECT_PATH}' not found"; fi
	mkdir -p "${CURRENT}"
	if ! is_empty "${CURRENT}";
	then error_exit "'${CURRENT}' is not empty"; fi
	if ! phone_cam_available;
	then if ! user_continues "No iPhone cam. Continue?"; then exit 1; fi
	else CAM=Nokia; fi
	if test "${CAM}" = "Nokia";
	then if user_continues "Switch to FaceTime cam?"; then CAM=FaceTime; fi; fi
}

function usage {
	cat <<USAGE >&2
usage: record project_name
USAGE
	exit 1;
}

function parse_options {
	declare optstring="h";
	declare optvar;

	while getopts "$optstring" optvar;do
	case "$optvar" in 
		h) usage;;
		*) usage;;
	esac;done
}

function main {
	parse_options "$@";
	shift $((OPTIND - 1));
	if test -z "$1"; then usage; fi
	setup "$@";
	start_recording;
	read -n 2 _unused;
	stop_recording;
	archive_current;
}

main "$@";
