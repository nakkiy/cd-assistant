_exec_cda() {
  READLINE_LINE="$(cda)"
  READLINE_POINT=${#READLINE_LINE}
}

CDA_BINDKEY="${CDA_BINDKEY:-\\ef}"

bind -x "\"$CDA_BINDKEY\":_exec_cda"
