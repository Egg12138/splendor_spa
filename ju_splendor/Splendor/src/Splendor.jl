include("play.jl")
include("const_data.jl")
include("utils.jl")
"""
使用Revise是为了保证再我已经`using Splendor`时我更改源文件仍然可以生效
AI目前是不会记牌的。
NOTICE: 或许我们可以让它会记牌？
"""
module Splendor
export greet, gamerun
function  greet()
	println("Hello world")
end


end # module Splendor

# TODO: git repo.

"""
# GameRun Main Loop
* reset Gems: [7,7,7,7,7]
# TODO: 解决一下玩家购买花费的自由度问题
"""
function gamerun()

	game = GameInit()
	# which is a matrix
	game
	# while !game_over(game)

		


	# end
end