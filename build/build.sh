#!/bin/sh


#
# Basic program information.
#
PROGRAM="$(basename ${0})"
PWD="$(pwd)"


#
# Options.
#
OPTSTR=""

CHAR_OPT_HELP="h"
opt_help="false"
OPTSTR+="${CHAR_OPT_HELP}"

CHAR_OPT_USAGE="u"
opt_usage="false"
OPTSTR+="${CHAR_OPT_USAGE}"

CHAR_OPT_BUILD="b"
opt_build="false"
STR_ARG_BUILD="ARTIFACT"
STR_ARG_BUILD_SRC="src"
STR_ARG_BUILD_DEB="deb"
STR_ARG_BUILD_RPM="rpm"
STR_ARG_BUILD_ALL="all"
DEFAULT_ARG_BUILD=""
arg_build="${DEFAULT_ARG_BUILD}"
OPTSTR+="${CHAR_OPT_BUILD}:"

CHAR_OPT_CLEAN="c"
opt_clean="false"
OPTSTR+="${CHAR_OPT_CLEAN}"

CHAR_OPT_FORCE="f"
opt_force="false"
OPTSTR+="${CHAR_OPT_FORCE}"

CHAR_OPT_VERBOSE="v"
opt_verbose="false"
OPTSTR+="${CHAR_OPT_VERBOSE}"

CHAR_OPT_PROJDIR="d"
opt_projdir="false"
STR_ARG_PROJDIR="PROJ_DIR"
DEFAULT_ARG_PROJDIR="${PWD}"
arg_projdir="${DEFAULT_ARG_PROJDIR}"
OPTSTR+="${CHAR_OPT_PROJDIR}:"

CHAR_OPT_NAME="N"
opt_name="false"
STR_ARG_NAME="NAME"
DEFAULT_ARG_NAME="dboeger1-dotfiles"
arg_name="${DEFAULT_ARG_NAME}"
OPTSTR+="${CHAR_OPT_NAME}"

CHAR_OPT_VERSION="V"
opt_version="false"
STR_ARG_VERSION="VERSION"
DEFAULT_ARG_VERSION="0.0.1"
arg_version="${DEFAULT_ARG_VERSION}"
OPTSTR+="${CHAR_OPT_VERSION}:"


#
# Usage statement.
#
USAGE="$(cat << END
Usage: %s {-%c | -%c | -%c %s | -%c} [-%c%c]
        [-%c %s] [-%c %s] [-%c %s]
    Run \"%s -%c\" for more help.
END
)"
USAGE=$(printf "${USAGE}"   \
    "${PROGRAM}"            \
    "${CHAR_OPT_HELP}"      \
    "${CHAR_OPT_USAGE}"     \
    "${CHAR_OPT_BUILD}"     \
    "${STR_ARG_BUILD}"      \
    "${CHAR_OPT_CLEAN}"     \
    "${CHAR_OPT_FORCE}"     \
    "${CHAR_OPT_VERBOSE}"   \
    "${CHAR_OPT_PROJDIR}"   \
    "${STR_ARG_PROJDIR}"    \
    "${CHAR_OPT_NAME}"      \
    "${STR_ARG_NAME}"       \
    "${CHAR_OPT_VERSION}"   \
    "${STR_ARG_VERSION}"    \
    "${PROGRAM}"            \
    "${CHAR_OPT_HELP}"      \
)

exit_usage()
{
    echo "${USAGE}"
    exit 0
}

exit_usage_err()
{
    echo "${USAGE}" 1>&2
    exit 1
}


#
# Help message.
#
HELP="$(cat << END
${USAGE}

Help options:

    -%c              Display this help message. This option overrides all other
                    options.

    -%c              Display a brief usage statement. This option overrides all
                    other options except for -%c.

Mode options:

    -%c %s     Build the specified artifacts. Sources and artifacts are
                    located within the project root directory, which can be
                    specified with -%c. The %s argument specifies which
                    artifacts to build. Valid values are "%s", "%s", "%s",
                    and "%s". "%s" produces a bundle of the sources. Both
                    "%s" and "%s" build the source bundle and a package of the
                    respective format. "%s" produces all of the previously
                    mentioned artifacts. If multiple instances of this option
                    are provided, all specified artifacts are built. By default,
                    if any conflicting artifacts are present, all builds fail
                    without making any changes. This behavior can be changed
                    with -%c.

    -%c              Clean all build artifacts from the project root directory,
                    which can be specified with -%c.

Common mode modifier options:

    -%c %s     Specify the project root directory. By default, the current
                    working directory is used.

    -%c %s         Specify the name string of the sources being built. The
                    default value is "%s".

    -%c %s      Specify the version string of the sources being built. The
                    default value is "%s".

Build mode modifier options:

    -%c              Force rebuilding and overwriting of existing artifacts in
                    the destination directory. Without this option, builds fail
                    in the presence of conflicting artifacts.

Informational options:

    -%c              Enable verbose output.
END
)"
HELP=$(printf "${HELP}"         \
    "${CHAR_OPT_HELP}"          \
    "${CHAR_OPT_USAGE}"         \
    "${CHAR_OPT_HELP}"          \
    "${CHAR_OPT_BUILD}"         \
    "${STR_ARG_BUILD}"          \
    "${CHAR_OPT_PROJDIR}"       \
    "${STR_ARG_BUILD}"          \
    "${STR_ARG_BUILD_SRC}"      \
    "${STR_ARG_BUILD_DEB}"      \
    "${STR_ARG_BUILD_RPM}"      \
    "${STR_ARG_BUILD_ALL}"      \
    "${STR_ARG_BUILD_SRC}"      \
    "${STR_ARG_BUILD_DEB}"      \
    "${STR_ARG_BUILD_RPM}"      \
    "${STR_ARG_BUILD_ALL}"      \
    "${CHAR_OPT_FORCE}"         \
    "${CHAR_OPT_CLEAN}"         \
    "${CHAR_OPT_PROJDIR}"       \
    "${CHAR_OPT_PROJDIR}"       \
    "${STR_ARG_PROJDIR}"        \
    "${CHAR_OPT_NAME}"          \
    "${STR_ARG_NAME}"           \
    "${DEFAULT_ARG_NAME}"       \
    "${CHAR_OPT_VERSION}"       \
    "${STR_ARG_VERSION}"        \
    "${DEFAULT_ARG_VERSION}"    \
    "${CHAR_OPT_FORCE}"         \
    "${CHAR_OPT_VERBOSE}"       \
)

exit_help()
{
    echo "${HELP}"
    exit 0
}


#
# Parse options.
#
while getopts "${OPTSTR}" OPT
do
    case "${OPT}" in
    "${CHAR_OPT_HELP}")
        opt_help="true"
        ;;
    "${CHAR_OPT_USAGE}")
        opt_usage="true"
        ;;
    "${CHAR_OPT_BUILD}")
        opt_build="true"
        arg_build="${OPTARG}"
        ;;
    "${CHAR_OPT_CLEAN}")
        opt_clean="true"
        ;;
    "${CHAR_OPT_FORCE}")
        opt_force="true"
        ;;
    "${CHAR_OPT_VERBOSE}")
        opt_verbose="true"
        ;;
    "${CHAR_OPT_PROJDIR}")
        opt_projdir="true"
        arg_projdir="${OPTARG}"
        ;;
    "${CHAR_OPT_NAME}")
        opt_name="true"
        arg_name="${OPTARG}"
        ;;
    "${CHAR_OPT_VERSION}")
        opt_version="true"
        arg_version="${OPTARG}"
        ;;
    "?")
        exit_usage_err
        ;;
    esac
done


#
# Validate and perform preemptive tasks.
#
if
    [ "${opt_help}"  != "true" ] &&
    [ "${opt_usage}" != "true" ] &&
    [ "${opt_build}" != "true" ] &&
    [ "${opt_clean}" != "true" ]
then
    printf "Must specify at least one: {-%c, -%c, -%c, -%c}\n" \
        "${CHAR_OPT_HELP}"  \
        "${CHAR_OPT_USAGE}" \
        "${CHAR_OPT_BUILD}" \
        "${CHAR_OPT_CLEAN}" 1>&2
    exit_usage_err
fi

if [ "${opt_help}" = "true" ]
then
    exit_help
fi

if [ "${opt_usage}" = "true" ]
then
    exit_usage
fi


#
# Validate remaining options.
#
if [ "${opt_build}" = "true" ] && [ "${opt_clean}" = "true" ]
then
    echo "-${CHAR_OPT_BUILD} and -${CHAR_OPT_CLEAN} are mutually exclusive." \
        1>&2
    exit_usage_err
fi

if
    [ "${opt_build}" = "true" ] &&
    [ "${arg_build}" != "src" ] &&
    [ "${arg_build}" != "deb" ] &&
    [ "${arg_build}" != "rpm" ] &&
    [ "${arg_build}" != "all" ]
then
    echo "Invalid -${CHAR_OPT_BUILD} argument: \"${arg_build}\"" 1>&2
    printf "Must be one of: {\"%s\", \"%s\", \"%s\", \"%s\"}\n" \
        "${STR_ARG_BUILD_SRC}" \
        "${STR_ARG_BUILD_DEB}" \
        "${STR_ARG_BUILD_RPM}" \
        "${STR_ARG_BUILD_ALL}" \
        1>&2
    exit_usage_err
fi

if [ "${opt_projdir}" = "true" ] && [ ! -d "${arg_projdir}" ]
then
    echo "Project directory does not exist: \"${arg_projdir}\"" 1>&2
    exit_usage_err
fi

# TODO: Validate arg_version.


#
# Build.
#
proj_src_dir="${arg_projdir}/src"

proj_build_dir="${arg_projdir}/build"

build_src_dir="${proj_build_dir}/src"
src_file_name="${arg_name}-${arg_version}.tar.gz"
src_file="${build_src_dir}/${src_file_name}"

build_deb_dir="${proj_build_dir}/deb"
deb_dir="${build_deb_dir}/DEB"

build_rpm_dir="${proj_build_dir}/rpm"
rpmbuild_dir="${build_rpm_dir}/rpmbuild"
rpmbuild_build_dir="${rpmbuild_dir}/BUILD"
rpmbuild_buildroot_dir="${rpmbuild_dir}/BUILDROOT"
rpmbuild_rpms_dir="${rpmbuild_dir}/RPMS"
rpmbuild_sources_dir="${rpmbuild_dir}/SOURCES"
rpmbuild_specs_dir="${rpmbuild_dir}/SPECS"
rpmbuild_srpms_dir="${rpmbuild_dir}/SRPMS"
rpmbuild_src_file="${rpmbuild_sources_dir}/${src_file_name}"

if [ "${opt_build}" = "true" ]
then
    mkdir -p ${build_src_dir}
    tar -c -C "${proj_src_dir}" -f "${src_file}" \
        --transform="s_^_${arg_name}-${arg_version}/_" \
        neovim tmux

    if [ "${arg_build}" = "deb" ] || [ "${arg_build}" = "all" ]
    then
        mkdir -p ${deb_dir}
    fi

    if [ "${arg_build}" = "rpm" ] || [ "${arg_build}" = "all" ]
    then
        mkdir -p \
            ${rpmbuild_build_dir} \
            ${rpmbuild_buildroot_dir} \
            ${rpmbuild_rpms_dir} \
            ${rpmbuild_sources_dir} \
            ${rpmbuild_specs_dir} \
            ${rpmbuild_srpms_dir}

        cp ${src_file} ${rpmbuild_src_file}

        sed \
            -e "s/^Name:\$/Name: ${arg_name}/" \
            -e "s/^Version:\$/Version: ${arg_version}/" \
            -e "s/^Source0:\$/Source0: ${rpmbuild_src_file}/" \
            ${build_rpm_dir}/name.spec \
            > ${rpmbuild_specs_dir}/${arg_name}.spec

        #rpmbuild --define "_topdir ${rpmbuild_dir}" -ba \
        #    ${rpmbuild_specs_dir}/${arg_name}.spec
    fi

    exit 0
fi

#
# Clean.
#
if [ "${opt_clean}" = "true" ]
then
    if [ -e "${rpmbuild_dir}" ]
    then
        if [ ! -d "${rpmbuild_dir}" ]
        then
            echo "Cannot clean non-directory: \"${rpmbuild_dir}\"" 1>&2
            exit 1
        fi

        rm -rf ${rpmbuild_dir}
    fi

    if [ -e "${deb_dir}" ]
    then
        if [ ! -d "${deb_dir}" ]
        then
            echo "Cannot clean non-directory: \"${deb_dir}\"" 1>&2
            exit 1
        fi

        rm -rf ${deb_dir}
    fi

    if [ -e "${build_src_dir}" ]
    then
        if [ ! -d "${build_src_dir}" ]
        then
            echo "Cannot clean non-directory: \"${build_src_dir}\"" 1>&2
            exit 1
        fi

        rm -rf ${build_src_dir}
    fi

    exit 0
fi
