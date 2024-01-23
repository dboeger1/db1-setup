--[[

    Core Key Maps

--]]


-- Mouse
vim.o.mouse = ''


-- Save/Quit
vim.api.nvim_set_keymap('n', '<Leader>q', ':q!<CR>', {noremap = true, silent = true})
vim.api.nvim_set_keymap('n', '<Leader>s', ':w<CR>',  {noremap = true, silent = true})
