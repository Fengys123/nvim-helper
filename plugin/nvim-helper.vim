" Initialize the channel
if !exists('s:calculatorJobId')
	let s:calculatorJobId = 0
endif

" The path to the binary that was created out of 'cargo build' or
" 'cargo build --release". This will generally be 'target/release/name'
" TODO: fix path
let s:bin = '/home/fys/fast/source/nvim-helper/target/debug/nvim-helper'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()

  if 0 == id
    echoerr "calculator: cannot start rpc process"
  elseif -1 == id
    echoerr "calculator: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:calculatorJobId = id 
 
    " --- Add the following line --- "
    call s:configureCommands() 
  endif
endfunction

" Initialize RPC
function! s:initRpc()
  if s:calculatorJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:calculatorJobId
  endif
endfunction

function! s:configureCommands()
  command! Clippy :call s:clippy()
  command! Test :call s:test()
endfunction

" Constants for RPC messages.
let s:Clippy = "clippy"
let s:Test = "trail_space"

function! s:clippy(...)
  call rpcnotify(s:calculatorJobId, s:Clippy)
endfunction

function! s:test( ... )
  call rpcnotify(s:calculatorJobId, s:Test)
endfunction

call s:connect()
