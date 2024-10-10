
local mark = require("harpoon.mark")
local ui = require("harpoon.ui")

--set leader key to space
vim.g.mapleader = " "

vim.opt.number = true

--keybindings
vim.api.nvim_set_keymap("n", "<Leader>m", ":lua require('harpoon.ui').toggle_quick_menu()<CR>", { noremap = true, silent = true })


vim.api.nvim_set_keymap("n", "<Leader>a", ":lua require('harpoon.mark').add_file()<CR>", { noremap = true, silent = true })


-- Load Packer
vim.cmd [[packadd packer.nvim]]

-- Packer plugin manager
require('packer').startup(function(use)
    use 'wbthomason/packer.nvim'  -- Packer can manage itself
    use 'neovim/nvim-lspconfig'  -- Collection of configurations for built-in LSP
    use 'theprimeagen/harpoon'  -- Harpoon for quick navigation
    use 'nvim-lua/plenary.nvim'  -- Required dependency for Harpoon
    -- Add Rust Analyzer
    use {
        'simrat39/rust-tools.nvim',
        config = function()
            require('rust-tools').setup({
                tools = {
                    autoSetHints = true,
                    inlay_hints = {
                        highlight = "SpecialComment",
                        prefix = "➤ ",  -- This is a function that will be called before
                        enabled = true,  -- Other options to enable/disable the hints
                    },
                },
                server = {
                    on_attach = function(_, bufnr)
                        -- Keybindings for LSP
                        local opts = { noremap=true, silent=true }
                        vim.api.nvim_buf_set_keymap(bufnr, 'n', '<Leader>rn', ':RustRunnables<CR>', opts)
                        vim.api.nvim_buf_set_keymap(bufnr, 'n', '<Leader>ha', ':RustHoverActions<CR>', opts) -- New keybinding for hover actions
                    end,
                },
            })
        end,
    }

    if packer_bootstrap then
        require('packer').sync()
    end
end)

