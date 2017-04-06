
@('ufind', './ufind') | %{
    Register-ArgumentCompleter -Native -CommandName $_ -ScriptBlock {
        param($wordToComplete, $commandAst, $cursorPosition)

        $command = '_ufind'
        $commandAst.CommandElements |
            Select-Object -Skip 1 |
            %{
                switch ($_.ToString()) {

                    'digraph' {
                        $command += '_digraph'
                        break
                    }

                    'help' {
                        $command += '_help'
                        break
                    }

                }
            }

        $completions = @()

        switch ($command) {

            '_ufind' {
                $completions = @('digraph', 'help', '-h', '-V', '--help', '--version')
            }

            '_ufind_digraph' {
                $completions = @('-d', '-h', '-V', '-c', '-f', '--description', '--help', '--version', '--convert', '--filter')
            }

            '_ufind_help' {
                $completions = @('-h', '-V', '--help', '--version')
            }

        }

        $completions |
            ?{ $_ -like "$wordToComplete*" } |
            Sort-Object |
            %{ New-Object System.Management.Automation.CompletionResult $_, $_, 'ParameterValue', $_ }
    }
}
