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
	((reach_target(game.p0)) || (reach_target(game.p1))) && 
	(game.p0.actcounter == game.p1.actcounter) ||
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

function pick_gems_success!(p::Player, game::Game, gems...)
	@assert length(gems) == 3 || length(gems) == 2  "你只能拿最多3，最少2个宝石"
	if max_pick_num(p) < length(gems)
		println("宝石存量即将超过", MAX_GEMS)
		return false
	end
	if !unique_ele(gems) 			# 拿同色
		@assert length(gems) == 2 "同色只能取两个" 
		if can_pick_double(game.gems, gems[1]) 
			player_gems_update!(p, gems[1], 2)
			gems_update!(game, gems[1], -2)
			true
		else 
			println("余量过少")
			false
		end
 	else
 		clr1, clr2, clr3 = gems						# 拿异色
 		if  can_pick_it(game, clr1) && can_pick_it(game, clr1) && can_pick_it(game, clr1)  
			player_gems_update!(p, gems[1], 1)
			player_gems_update!(p, gems[2], 1)
			player_gems_update!(p, gems[3], 1)
			gems_update!(game, gems[1], -1)
			gems_update!(game, gems[2], -1)
			gems_update!(game, gems[3], -1)
			true
 		else
 			println("余量不足")
 			false
 		end
	end
end














end #module Play