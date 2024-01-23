#!/bin/sh


# Options.
OPTSTRING=":"

CHAR_OPT_SOURCE_ROOT_DIR="d"
OPT_SOURCE_ROOT_DIR=false
DEFAULT_ARG_SOURCE_ROOT_DIR=""
ARG_SOURCE_ROOT_DIR="${DEFAULT_ARG_SOURCE_ROOT_DIR}"
OPTSTRING+="${CHAR_OPT_SOURCE_ROOT_DIR}:"


# Usage
PROGRAM="$(basename ${0})"
USAGE="Usage: ${PROGRAM} -${CHAR_OPT_SOURCE_ROOT_DIR} <source root directory>"


# Parse arguments.
while getopts "${OPTSTRING}" OPT; do
    case "${OPT}" in
        "${CHAR_OPT_SOURCE_ROOT_DIR}")
            OPT_SOURCE_ROOT_DIR=true
            ARG_SOURCE_ROOT_DIR="${OPTARG}"
            ;;
        ":")
            echo "Missing argument for option: -${OPTARG}" >&2
            echo "${USAGE}" >&2
            exit 1
            ;;
        "?")
            echo "Invalid option: -${OPTARG}" >&2
            echo "${USAGE}" >&2
            exit 1
            ;;
    esac
done


# Validate arguments.
if [ "${OPT_SOURCE_ROOT_DIR}" != "true" ]; then
    echo "Missing required argument: -${CHAR_OPT_SOURCE_ROOT_DIR}" >&2
    echo "${USAGE}" >&2
    exit 1
fi

if [ ! -d "${ARG_SOURCE_ROOT_DIR}" ]; then
    echo "Not a directory: \"${ARG_SOURCE_ROOT_DIR}\"" >&2
    echo "${USAGE}" >&2
    exit 1
fi

CARGO_TOML="${ARG_SOURCE_ROOT_DIR}/cargo/Cargo.toml"
if [ ! -f "${CARGO_TOML}" ]; then
    echo "Missing Cargo.toml file: \"${CARGO_TOML}\"" >&2
    echo "${USAGE}" >&2
    exit 1
fi


# Extract information about sources.
NAME=$( \
    grep "name = " ${CARGO_TOML} | \
    head -1 | \
    awk '{ print $3 }' | \
    tr -d '"')

VERSION=$( \
    grep "version = " ${CARGO_TOML} | \
    head -1 | \
    awk '{ print $3 }' | \
    tr -d '"')

NAME_VERSION="${NAME}-${VERSION}"


# Create source archive.
TEMP_DIR="${TMPDIR:-/tmp}"
if [ ! -d "${TEMP_DIR}" ]; then
    echo "Missing temporary directory for builds: \"${TEMP_DIR}\"" >&2
    echo "${USAGE}" >&2
    exit 1
fi

ARCHIVE_DIR="${TEMP_DIR}/${NAME_VERSION}"
mkdir -p ${ARCHIVE_DIR}

FILE="${ARCHIVE_DIR}/${NAME_VERSION}.tar.gz"
echo "Creating source archive: \"${FILE}\"..."

tar \
    --create \
    --gzip \
    --directory=${ARG_SOURCE_ROOT_DIR} \
    --exclude=.git \
    --exclude=cargo/target \
    --file=${FILE} \
    --transform=s#^.#${NAME_VERSION}# \
    .
