module 	Play

export Game, GameInit, game_over, buy_from_store!, Player, pick_gems_success!, buy_card_success!
include("utils.jl")
function GameInit()
	d1, d2, d3 = shuffle_decks_from_cardspool()
	game = Game(
	gems_area_reset(),
	nobles_shuffle(),
	# shuffle_decks_from_cardspool()	
	d1, d2, d3,
	cards_store_reset!(d1, d2, d3),	
	0, 
	PlayerInitial(0),
	PlayerInitial(1),
	)
	game
end


function game_over(game::Game)  
	((reach_target(game.p0)) || (reach_target(game.p1))) || 
	(!can_uncover(game.deck1) && !can_uncover(game.deck2) && !can_uncover(game.deck3) )  
end



"store is a matrx. deck is the specific dekc of levelX, return the bought one"
function buy_from_store!(game::Game, level, idx)
	bought = _remove_one_from_store!(game.cards_store, level, idx)
	if level == 1 deck = game.deck1 
	elseif level == 2 deck = game.deck2
	else deck = game.deck3	end
	if can_uncover(deck)
		game.cards_store[idx, level] = _uncover!(deck)	
	end
	bought
end


function buy_card_success!(p::Player, game::Game, level, idx)
		# 这一层检查重新放到main_loop()中
	if card_available(game.cards_store, level, idx)
		if affortable(p, game.cards_store[idx, level])
			card = buy_from_store!(game, level, idx)
			cost = card[1:5]
			pay!(p, game, cost)
			bought_a_new_card!(p, card)
		else
			println(p.gems, game.cards_store[idx, level])
			println("华为助手：你买不起！")
		end
		true
	else
		println("华为助手：此位已售罄！")
		false
	end
end

function pick_gems_success!(p::Player, game::Game, gems::Vector{UInt8})
	len = length(gems)
	@assert len == 3 || len == 2  "你只能拿最多3，最少2个宝石"
	if max_pick_num(p) < length(gems)
		println("宝石存量即将超过", MAX_GEMS)
		return false
	end

	if !unique_ele(gems) 			# 拿同色
		if len != 2
			false
		end
		if can_pick_double(game.gems, gems[1]) 
			player_gem_update!(p, gems[1], 2)
			gems_update!(game, gems[1], -2)
			true
		else 
			println("余量过少")
			false
		end
	elseif len == 2
		false
	end


	# 异色, 2/3个
	if  can_pick_it(game.gems, gems[1]) && can_pick_it(game.gems, gems[2]) && can_pick_it(game.gems, gems[3])
		player_gem_update!(p, gems[1], 1)
		player_gem_update!(p, gems[2], 1)
		player_gem_update!(p, gems[3], 1)
		gems_update!(game, gems[1], -1)
		gems_update!(game, gems[2], -1)
		gems_update!(game, gems[3], -1)
	else
		println("余量不足")
		false
	end
	true
	end


function short_print_gems(clrid)
	print(GEM_COLORS[clrid]," ")
end

function handle_action!(player::Player, game::Game)
	#IMPL: Finish
	# action_over = false
	# while action_over
	while true
		print("p$(player.id)\t:")
		command = readline()
		buy_matched = match(buy_regex, command)
		pick_matched = match(pick_regex, command) 
		if buy_matched !== nothing
			level, idx = parse_buy(buy_matched)
			if buy_card_success!(player, game, level, idx)
				println("[LOG]Bought successfully")
				break 
			else
				println("[WARN]Cannot buy the card")
			end
		elseif pick_matched !== nothing
			gems = parse_pick(pick_matched)
			if pick_gems_success!(player, game, gems)
				print("[LOG]Picked ")
				foreach(g->short_print_gems(g), gems)
				println("")
				break
			else 
				println("[WARN]Cannot pick these gems")
			end
			break
		else 
			println("Failed to parse the command. Enter again:")
		end
	end
end












end #module Play