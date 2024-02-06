--[[

    Telescope Fuzzy Finder


    References:

        https://github.com/nvim-telescope/telescope.nvim
        https://github.com/nvim-lua/plenary.nvim
        https://github.com/nvim-telescope/telescope-fzf-native.nvim

--]]


-- Lazy Spec
return {
    'nvim-telescope/telescope.nvim',
    enabled = true,
    dependencies = {
        'nvim-lua/plenary.nvim',
        'nvim-telescope/telescope-fzf-native.nvim',
    },
    config = function()
        -- Dependency Modules
        local telescope = require('telescope')

        -- Setup
        telescope.setup({})

        -- Load FZF Extension
        telescope.load_extension('fzf')

        -- Key maps.
        vim.api.nvim_set_keymap('n', '<Leader>F', ':Telescope<CR>', {noremap = true, silent = true})
        vim.keymap.set('n', '<Leader>ff', require('telescope.builtin').find_files, { desc = 'Find [F]iles' })
        vim.keymap.set('n', '<Leader>fg', require('telescope.builtin').live_grep, { desc = 'Live [G]rep' })
        vim.keymap.set('n', '<Leader>fw', require('telescope.builtin').grep_string, { desc = 'Find [W]ord' })
    end,
}
