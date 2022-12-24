module Tst
	include("../src/utils.jl")
	ps1 = "pick 1,2,3"
	ps2 = " p 1,2,3,"
	ps3 = " p 1,2,"
	ps4 = "p 1,2"
	ps5 = "pic 1,2,3,4"
	psm1 = match(pick_regex, ps1)	
	psm2 = match(pick_regex, ps2)	
	psm3 = match(pick_regex, ps3)	
	psm4 = match(pick_regex, ps4)	
	psm5 = match(pick_regex, ps5)	
	if psm1 !== nothing && psm2 !== nothing && psm3 !== nothing && psm4 !== nothing && psm5 !== nothing
		@assert parse_pick(psm1) == (1,2,3)
		@assert parse_pick(psm2) == (1,2,3)
		@assert parse_pick(psm3) == (1,2)
		@assert parse_pick(psm4) == (1,2)
		@assert parse_pick(psm5) == (1,2,3)
	end

end