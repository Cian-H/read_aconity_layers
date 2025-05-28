# Lists available just commands
default:
    @just --list

# Documentation commands - delegates to docs/justfile
docs command *args:
    cd docs && just {{command}} {{args}}
