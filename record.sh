#!/usr/bin/env bash

RECORDINGS_PATH="/Volumes/T7/code_videos/Rushes"
SCREEN_DISPLAY=Capture
DEVICE_1=Nokia
THRESHOLD=5 #The script will refuse running if 
# there isn't at least 'THRESHOLD' space left
# (in Gib) on the target filesystem


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

# This function computes the following numbered
# directory under <project_path> where the current
# session will be stored
function compute_next_folder {
	declare last;
	last="$(ls -1 "${PROJECT_PATH:?}" | grep -E '[0-9]' | sort -h | tail -n 1)";
	last="$((last + 1))";
	next_folder="${last}"
}

# This function takes the directory <project_path>/current
# and archives it to a numbered directory automatically. It
# computes the last numbered directory and increases the 
# count automatically
function archive_current {
	declare next_folder;
	compute_next_folder;
	if test -f "${CURRENT:?}/pids";
	then error_exit "programm still running";  fi
	mv -i "${CURRENT:?}" "${PROJECT_PATH:?}/${next_folder}";
}

function error_exit {
	echo "$@" >&2;
	exit 1;
}

# To list the available devices for the avfoundation backend
# ffmpeg -f avfoundation -list_devices true -i ""

# Launch the screen and face recording in background
# Stores the pid in a file so they can be killed later on
function start_recording {
	if ! ffmpeg -f avfoundation -audio_device_index 1 -i "${SCREEN_DISPLAY}" "${CURRENT}/screen.mp4" &
	then exit 1; fi
	echo $! > "${PIDS_FILE}"
	if ! ffmpeg -f avfoundation -framerate 30 -i "${CAM}" "${CURRENT}/face.mp4" &
	then read p < "${PIDS_FILE}" ; kill $p; exit 1; fi
	echo $! >> "${PIDS_FILE}"
}

# Checks if there are more video displays than only the camera and the screen
# For now, it only try to grep my iPhone's name (Nokia de Thibault) and returns
# the result of the operation
function phone_cam_available {
	if ffmpeg -f avfoundation -list_devices true -i "" 2>&1 | grep "${DEVICE_1}" >/dev/null;
	then return 0; else return 1; fi
}

function stop_recording {
	if ! test -f "${PIDS_FILE}"; then echo "file: '${PIDS_FILE}' not found"; exit 1; fi
	while read p; do kill "$p"; done < "${PIDS_FILE}"
	rm "${PIDS_FILE}";
}

# To be used to check if the directory <project_path>/current is 
# empty
function is_empty {
	if test "$(ls "$1" | wc -l)" -eq "0";
	then return 0; else return 1; fi
}

# To be used when the user needs to choose between two options
function user_continues {
	read -p "${1} [y/n]" -n 1;
	case "$REPLY" in 
		[Y/y]) echo ""; return 0;;
		*) echo ""; return 1;;
	esac
}

# Checks for existence of project path, number of cameras 
# and sets up variable for recording
function setup {
	declare space;
	CAM=FaceTime
	PROJECT_NAME="$1"
	PROJECT_PATH="${RECORDINGS_PATH}/${PROJECT_NAME}"
	CURRENT="${PROJECT_PATH}/current"
	LOGS="${PROJECT_PATH}/recording_sizes.log"
	FAIL="${PROJECT_PATH}/fail"; # To be used only if leftovers of 
	# a previous recording are found
	PIDS_FILE="${CURRENT}/pids"
	if ! test -d "${PROJECT_PATH}";
	then error_exit "'${PROJECT_PATH}' not found"; fi
	mkdir -p "${CURRENT}"
	if ! is_empty "${CURRENT}";
	then 
		if ! user_continues "'${CURRENT}' directory not empty, would you like to continue?";
		then exit 1; fi
		if ! user_continues "Would you like to archive '${CURRENT}' into '${FAIL}'?";
		then rm -rf "${CURRENT}";
		else mv "${CURRENT}" "${FAIL}"; 
		fi
		mkdir -p "${CURRENT}";
	fi
	available_space;
	if ! user_continues "Available space is: ${space:?} Gib. Continue?"; then exit 1; fi
	if ! phone_cam_available;
	then if ! user_continues "No iPhone cam. Continue?"; then exit 1; fi
	else CAM="${DEVICE_1}"; fi
	if test "${CAM}" = "${DEVICE_1}";
	then if user_continues "Switch to FaceTime cam?"; then CAM=FaceTime; fi; fi
}

function usage {
	cat <<USAGE >&2
usage: record project_name
USAGE
	exit 1;
}

function available_space {
	space="$(df -g ${CURRENT:?} | tail -n 1 | awk -F ' ' '{ print $4 }')";
	if test "$space" -lt "${THRESHOLD:?}";
	then cat <<-WARNING >&2
		There are currently less than ${THRESHOLD:?} Gib left on the target device.
		Please empty your drive, before attempting a new recording
	WARNING
	fi
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

function recording_message {
	cat <<-MESSAGE
	Recording is about to start.
	Press one key to start recording
	When recording, press one key to stop recording.
	MESSAGE
}

function create_log_file {
	if ! test -f "${LOGS:?}";
	then touch "${LOGS}"; chmod 644 "${LOGS}"; fi
}

function log_size {
	declare next_folder;
	create_log_file;
	size="$(du -h ${CURRENT} | awk -F ' ' '{ print $1 }')"
	echo "The recording session took ${size}";
	compute_next_folder;
	session_date="$(date "+%Y-%m-%d %H:%M:%S")";
	echo "${session_date:?} - Session ${next_folder:?} : ${size:?}" >> "${LOGS:?}"
}

function main {
	parse_options "$@";
	shift $((OPTIND - 1));
	if test -z "$1"; then usage; fi
	setup "$@";
	recording_message;
	read -n 1 _unused;
	start_recording;
	read -n 1 _unused;
	stop_recording;
	sleep 3; # Ugly but need to ensure that the output from the ffmpeg bg
	# processes have time to print on stdout
	log_size;
	archive_current;
}

main "$@";
