#!/usr/bin/env sh
set -eu

script_root="${SHELL_SCRIPT_ROOT:-scripts}"

find "$script_root" -type f -name '*.sh' -print |
    while IFS= read -r script; do
        interpreter="$(sed -n '1p' "$script")"
        case "$interpreter" in
        '#!/usr/bin/env sh')
            sh -n "$script"
            ;;
        '#!/usr/bin/env bash')
            bash -n "$script"
            ;;
        *)
            echo "unsupported shell shebang in ${script}: ${interpreter}" >&2
            exit 1
            ;;
        esac
    done
