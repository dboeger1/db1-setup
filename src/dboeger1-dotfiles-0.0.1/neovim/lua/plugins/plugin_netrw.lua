--[[

    Netrw File Explorer


    References:

        :help netrw

--]]


-- Visual
vim.g.netrw_banner = 0    -- disable banner
vim.g.netrw_liststyle = 3 -- tree view


-- Window Splitting
vim.g.netrw_browse_split = 0 -- open selected file in current window


-- Open Explorer
vim.api.nvim_set_keymap('n', '<Leader>e', ':Explore<CR>', {noremap = true, silent = true})
