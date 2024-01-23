--[[

    Tree-Sitter Syntax Parsing Engine


    References:

        :help treesitter.txt
        :help nvim-treesitter-textobjects

        https://github.com/nvim-treesitter/nvim-treesitter
        https://github.com/nvim-treesitter/nvim-treesitter-textobjects

--]]


return {
    'nvim-treesitter/nvim-treesitter',
    enabled = true,
    dependencies = {
        'nvim-treesitter/nvim-treesitter-textobjects',
    },
    opts = {
        ensure_installed = {
            'vim',
            'help',
            'lua',
            'bash',
            'c',
            'cpp',
            'rust',
        },
        auto_install = false,
        highlight = { enable = true },
    },
}
