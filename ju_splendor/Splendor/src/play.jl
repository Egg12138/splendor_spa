include("utils.jl")
mutable struct Game
	gems::Vector{UInt8}			 		# 5-elements of Vec{u8}
	nobles::Vector{Vector{UInt8}}		 		# 3-elements of Vec{Vec{u8}}
	# decks::Vector{Vector{Vector{UInt8}}} # 因为长度不等
	deck1::Vector{Vector{UInt8}} 		# 40-elements of Vec{Vec{u8}}
	deck2::Vector{Vector{UInt8}} 		# 30-elements of Vec{Vec{u8}}
	deck3::Vector{Vector{UInt8}} 		# 20-elements of Vec{Vec{u8}}
	cards_store::Matrix{Vector{UInt8}}  # 4*3 Matrix, cards_store[:, N] to get Nth level cards Vec{UInt8}
	turn::UInt64 						# Game turn number
	p0::Player
	p1::Player
end

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

"store is a matrx. deck is the specific dekc of levelX"
function buy_from_store!(game::Game, level, idx)
	_remove_one_from_store!(game.cards_store, level, idx)
	if can_uncover(eval(Meta.parse("game.deck$level")))
		game.cards_store[idx, level] = _uncover!(eval(Meta.parse("game.deck$level")))	
	end
end