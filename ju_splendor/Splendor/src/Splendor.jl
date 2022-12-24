"""
使用Revise是为了保证再我已经`using Splendor`时我更改源文件仍然可以生效
AI目前是不会记牌的。
NOTICE: 或许我们可以让它会记牌？
"""
module Splendor
include("Play.jl")
export gamerun, main_loop

using .Play
"""
# GameRun Main Loop
* reset Gems: [7,7,7,7,7]
# TODO: 解决一下玩家购买花费的自由度问题
"""
function gamerun()

	game = Play.GameInit()
	# which is a matrix
	game
end

function show_all(game::Game)
	Play.show_cards_nobles(game.cards_store, game.turn)
	Play.show_available_gems(game.gems)
	Play.show_cards_nobles(game.nobles)
	Play.show_players(game.p0, game.p1)
end

function main_loop(game::Game)
	while !game_over(game)
		game.turn += 1
		show_all(game)
		Play.handle_action!(game.p0, game)
		show_all(game)
		Play.handle_action!(game.p1, game)
	end
	if Play.winner(game.p0, game.p1) == game.p0 
		0 
	else 
		1
	end

end

function interact_test_loop(game::Game)
	while !game_over(game)
		game.turn += 1
		show_all(game)	
		print("p0: ")
		command = readline()
		cmd_chars = Vector{Char}(command)
		lv, id = @. parse(Int, cmd_chars)
		if buy_card_success!(game.p0, game, lv, id)
			Play.handle_noble_sponse!(game.p0, game)
			show_all(game)
			print("p1: ")
			command = readline()
			cmd_chars = Vector{Char}(command)
			lv, id = @. parse(Int, cmd_chars)
			Play.buy_card_success!(game.p1, game, lv, id)
			Play.handle_noble_sponse!(game.p1, game)
		end	
		# other_regx = 命令联想启动  
		# handle_action!(game, p0)	
		# handle_action!(game, p1)	
		# TODO parser
		# TODO 添加测试用的决策函数
	end
		
	if Play.winner(game.p0, game.p1) == game.p0 
		0 
	else 
		1
	end
end

end


