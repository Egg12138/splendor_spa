module Tst
	include("../src/Splendor.jl")
	include("../src/utils.jl")
	import .Splendor
	games_area = gems_area_reset()
	nobles_area = nobles_reset()
	(deck1, deck2, deck3) = shuffle_decks_from_cardspool()
	cards_store = cards_store_reset!(deck1, deck2, deck3)
	println("\n\ncards store: $cards_store")
	println("\n LV1 :$(cards_store[:,1])")
	println("\n LV2 :$(cards_store[:,2])")
	println("\n LV3 :$(cards_store[:,3])")
	println(games_area)
	for i in 1:40 
		buy_store_update!(cards_store, deck1, 1, rand((1, 4)));
		println("\n LV1 :$(cards_store[:,1])")
	end
	Splendor.greet()
	p0 = PlayerInitial(0);
	p1 = PlayerInitial(1);
	println(p0)
end