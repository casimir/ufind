function __fish_using_command
    set cmd (commandline -opc)
    if [ (count $cmd) -eq (count $argv) ]
        for i in (seq (count $argv))
            if [ $cmd[$i] != $argv[$i] ]
                return 1
            end
        end
        return 0
    end
    return 1
end

complete -c ufind -n "__fish_using_command ufind" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind" -f -a "digraph"
complete -c ufind -n "__fish_using_command ufind" -f -a "help"
complete -c ufind -n "__fish_using_command ufind" -f -a "help"
complete -c ufind -n "__fish_using_command ufind digraph" -s c -d "Converts a digraph sequence or a character to the other"
complete -c ufind -n "__fish_using_command ufind digraph" -s f -d "Prints information about matching digraphs"
complete -c ufind -n "__fish_using_command ufind digraph" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind digraph" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind digraph" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind digraph" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind help" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind help" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind help" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind help" -s V -l version -d "Prints version information"
complete -c ufind -n "__fish_using_command ufind help" -s h -l help -d "Prints help information"
complete -c ufind -n "__fish_using_command ufind help" -s V -l version -d "Prints version information"
