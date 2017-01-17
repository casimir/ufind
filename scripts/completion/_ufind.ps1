
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

                    'help' {
                        $command += '_help'
                        break
                    }

                    'help' {
                        $command += '_help'
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
                $completions = @('digraph', 'help', 'help', 'help', 'help', '-h', '-V', '-h', '-V', '-h', '-V', '-h', '-V', '--help', '--version', '--help', '--version', '--help', '--version', '--help', '--version')
            }

            '_ufind_digraph' {
                $completions = @('-c', '-f', '-h', '-V', '-h', '-V', '-h', '-V', '-h', '-V', '--convert', '--filter', '--description', '--help', '--version', '--help', '--version', '--help', '--version', '--help', '--version')
            }

            '_ufind_help' {
                $completions = @('-h', '-V', '-h', '-V', '-h', '-V', '-h', '-V', '--help', '--version', '--help', '--version', '--help', '--version', '--help', '--version')
            }

            '_ufind_help' {
                $completions = @('-h', '-V', '-h', '-V', '-h', '-V', '--help', '--version', '--help', '--version', '--help', '--version')
            }

            '_ufind_help' {
                $completions = @('-h', '-V', '-h', '-V', '--help', '--version', '--help', '--version')
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
