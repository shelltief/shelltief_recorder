#!/usr/bin/env bash

RECORDINGS_PATH="/Volumes/T7/code_videos/Rushes"
SCREEN_DISPLAY=Capture

function archive_current {
	declare field_width=5;
	declare last;
	last="$(ls -1 | grep -E '[0-1]' | tail -n 1)";
	last="$(echo $last | bc --ibase=2)";
	last="$((last + 1))";
	last="$(echo $last | bc --obase=2)"
	if test "${#last}" -gt "${field_width}";
	then error_exit "wrapped"; fi
	last="$(printf "%0${field_width}d\n" "$last")";
	if test -f "${CURRENT}/pids";
	then error_exit "programm still running";  fi
	mv "${CURRENT}" "${last}"
}

function error_exit {
	echo "$@" >&2;
	exit 1;
}

function list_devices {
	ffmpeg -f avfoundation -list_devices true -i ""
}

function start_recording {
	if ! ffmpeg -f avfoundation -audio_device_index 1 -i "${SCREEN_DISPLAY}" "${CURRENT}/screen.mp4" &
	then exit 1; fi
	echo $! > "${PIDS_FILE}"
	if ! ffmpeg -f avfoundation -framerate 30 -i "${CAM}" "${CURRENT}/face.mp4" &
	then read p < "${PIDS_FILE}" ; kill $p; exit 1; fi
	echo $! >> "${PIDS_FILE}"
}

function phone_cam_available {
	if ffmpeg -f avfoundation -list_devices true -i "" 2>&1 | grep Nokia >/dev/null;
	then return 0; else return 1; fi
}

function stop_recording {
	if ! test -f "${PIDS_FILE}"; then echo "file: '${PIDS_FILE}' not found"; exit 1; fi
	while read p; do kill "$p"; done < "${PIDS_FILE}"
	rm "${PIDS_FILE}";
}

function is_empty {
	if test "$(ls "$1" | wc -l)" -eq "0";
	then return 0; else return 1; fi
}
function user_continues {
	read -p "${1} [y/n]" -n 1;
	case "$REPLY" in 
		[Y/y]) return 0;;
		*) return 1;;
	esac
}

function setup {
	CAM=FaceTime
	if test -z "$1"; then error_exit "No Project Name provided"; fi
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

function main {
	setup "$@"
	start_recording
	read -n 3 _unused
	stop_recording
	archive_current
}

main "$@";
