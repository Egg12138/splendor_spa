include("const_data.jl")
# 状态机操作函数定义
# Julia, Fortran, Matlab, R等语言采用数组的列优先内存访问
"""
# 组成
## 游戏状态机
## 判定函数
 * 牌堆是否翻空
 * 黄金是否拿完(`enough`)
 * 这个玩家是否触发最终回条件(`reach_target`)
 * 谁获胜了
 * 玩家能够继续预约
 * 玩家可以`pick`的宝石数(变体规则：不把金币算宝石，**之后为了促进玩家流通金币，可以引入无可获得金币不可逾越的条件约束**)
 * 是否买得起

## 状态机更新函数
 * 初始化牌区
 * 初始化贵族
 * 玩家赢得贵族
 * 玩家加分
 * 玩家完成行动加行动数
 * 玩家申请预约一张
 * 玩家试图购买一张
 * 玩家试图拿取单色宝石
 * 玩家试图拿取多色宝石
 * 
其中用了`@inbounds`等忽略对边界的检查。
元素的存储基本以`UInt8`的类型存储。

**性能提高**:
有非常多地方可以进一步优化，暂时没有时间和必要性进行，比如将`Vector{Vector{T}}`改为Matrix以及将我们已经搞好的按列顺位的常量矩阵作为固定的牌池来读取。
**组织重构**:
可以发现这个项目组织的非常凌乱，需要进一步重构和优化
因为Player在这里承担了状态机功能，所以它必须是可变的
"""
mutable struct Player
	id::UInt8
	"在这里，我们对规则做出调整，我们只限制总的宝石不能超过10个，把黄金排除宝石之列，这样可以少建一些字段"
	gems::Vector{UInt8}
	"发展卡颜色数: bought[1] => 绿卡数, bought[2] => 白卡数... 1,2,3,4,5 => GREEN, WHITE, BLUE, BLACK, RED"
	bought::Vector{UInt8}		# bought + gems
	golds::UInt8
	scores::UInt8
	"在第一次比较平分时，发展卡数量更少的玩家获胜"
	cards_num::UInt8
	reserved_cards::Vector{UInt8}
	reserved_num::UInt8
end
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


PlayerInitial(id) = Player(id, clrmap(), clrmap_init(), 0, 0, 0, res_init(), 0)
function score_add!(p::Player, score)  
	p.scores += score
end

@enum Action Pick=1 Buy=2 
using Random
# 判定函数：
card_available(store::Matrix{Vector{UInt8}}, level, idx) = store[idx, level][LVLIDX] != 0
can_uncover(deck::Vector{Vector{UInt8}}) = length(deck) != 0
can_reserve(player::Player) = length(player.reserved_num) < MAX_RESERVE
can_pick_double(gems::Vector{UInt8}, clridx) = gems[clridx] > 4 
can_pick_it(gems::Vector{UInt8}, clridx) = gems[clridx] >= 1
unique_ele(sq::Vector{UInt8}) = length(Set(sq)) == length(sq)
reach_target(player::Player) = player.scores >= TARGET
enough(n) = n > 0 
# turn_over(p1::Player, p2::Player) = p1.actcounter == p2.actcounter
# 后手有一点点的微弱补偿
function winner(p1::Player, p2::Player)
	( p1.scores > p2.scores) ?
	 ( p1 ) : ( (p1.scores < p2.scores) ? 
	 				( p2 ) : ( p1.cards_num < p2.cards_num ) ? (p1) : (p2) )
end

function handle_noble_sponse!(player::Player, game::Game)
	for (id, noble) in enumerate(game.nobles)
		delta_vec = _Δrequirement(convert(Vector{Int}, player.bought), noble)
		if sum(delta_vec) <= 0
			println("[LOG]p$(player.id) won the noble[$id], got 3 score.")
			won_a_noble!(player)
			deleteat!(game.nobles, id)
		end
	end
end

# 暂时不考虑黄金
"一种解决办法是提供给玩家有限的支付方式的选项，但是这样让应该会AI比较难受"
function affortable(player::Player, card::Vector{UInt8})
	delta = _Δrequirement(convert(Vector{Int64}, player.gems+player.bought), card[1:5])
	sum(delta) <= 0 
end

# private functions
@fastmath function _Δrequirement(data::Vector{Int64}, req::Vector{UInt8})
	@assert (length(data) == length(req))
	Δ = req - data
	Δ[Δ.<=0] .= 0
	Δ
end

# 考虑黄金
function _affort_considering_gold(player::Player,card::Vector{UInt8})
	delta =  _Δrequirement(player.gems, card)
	player.golds >= sum(delta)
end


function max_pick_num(player)
	gemsnum = sum(player.gems)
	(gemsnum <= MAX_GEMS) ? 3 : (MAX_GEMS - gemsnum)
end
# 状态机更新

gems_area_reset() = GEMFULL_NUM * ones(UInt8, 5)
"确认买得起,进行支付和购买后，更新玩家其他信息"
function bought_a_new_card!(p::Player, card::Vector{UInt8})
	p.cards_num += 1
	p.bought[card[COLORIDX]] += 1
	score_add!(p, card[SCOREIDX])
end
function pay!(p::Player, g::Game, costs::Vector{UInt8})
# costs is a 5-elements vector of uint8
	after_paid_by_boughtcards = _Δrequirement(convert(Vector{Int64}, p.bought), costs) # 付的掉的位为0，余下的位为正
	gems_area_update!(g, after_paid_by_boughtcards)	
	gems_area_update!(p, -after_paid_by_boughtcards)
end
function won_a_noble!(player::Player)
	score_add!(player, NOBLE_BONUS)
end



"""
关于**2维数据的结构**:
实操是按列，这里按行
[ score, action_count, reserved: Matrix{Int64}, reserved_num: Int64, gems: Vector{Int64}, gems_num为gems加和, gold_num, 
"""
function nobles_shuffle() 
	shuffled_nobles = shuffle(noblesdeck)
	[
	popfirst!(shuffled_nobles),
	popfirst!(shuffled_nobles),
	popfirst!(shuffled_nobles),
	]
end

"""
这种思路是向量化向量化操作，效率很低，之后改成纯矩阵操作加上按列读取
"""
@inline function shuffle_decks_from_cardspool()
	deck1 = convert(Vector{Vector{UInt8}}, shuffle(@inbounds cardsdeck[1:40]))
	deck2 = convert(Vector{Vector{UInt8}}, shuffle(@inbounds cardsdeck[41:70]))
	deck3 = convert(Vector{Vector{UInt8}}, shuffle(@inbounds cardsdeck[71:90]))
	[deck1, deck2, deck3]
end




""" 将一张卡填充为0完成remove
这里不能删去他们，因为
返回被卖的卡
"""
function _remove_one_from_store!(store, level, idx)
	bought = store[idx, level]
	store[idx, level] = zeros(UInt8, 8)
	bought
end

function _a_new_card_uncovered!(store, level, idx, newcard::Vector{UInt8})
	store[idx, level] = newcard
end

function _uncover!(tile::Vector{Vector{UInt8}})
	pop!(tile)
end

#==仅用于初始化，所以不需要检查==#
function uncover4!(tile::Vector{Vector{UInt8}})
	[
	popfirst!(tile),
	popfirst!(tile),
	popfirst!(tile),
	popfirst!(tile),
	]
end

"""
`cards_store_reset` 会返回一个矩阵,`store = cards_store_reset(d1, d2, d3)`, store[1,:]为lv1牌，store[X,:][Y]为X级牌第Y张
	若牌拿完则具体位置为[0,0,0]
"""
function cards_store_reset!(d1, d2, d3)
	println("Store init...")
	
	level1 = uncover4!(d1)
	level2 = uncover4!(d2)
	level3 = uncover4!(d3)
	[level1 level2 level3]
end

function gems_area_update!(game::Game, δmap::Vector{Int})
	@assert length(δmap) == 5 "delta Gems should be a 5-elements vector of u8 "
	game.gems += δmap
end 

function gems_update!(game::Game, clr, δ)
	@assert 1 <= clr <= 5 "[gems update]Color index should be in [1:5]"
	game.gems[clr] += δ
end

function gems_area_update!(p::Player, δmap::Vector{Int})
	@assert length(δmap) == 5 "delta Gems should be a 5-elements vector of u8 "
	p.gems += δmap
end
function player_gem_update!(p::Player, clr, δ)
	@assert 1 <= clr <= 5 "[player update]Color index should be in [1:5]"
	p.gems[clr] += δ
end


"Initialize the color-num map vector"
clrmap() = ones(UInt8, 5) * 30
clrmap_init() = zeros(UInt8, 5)
res_init() = zeros(UInt8, MAX_RESERVE)
global_gems_init() = 7 * ones(UInt8, 5)





# TODO: 纵向分割的排版十分重要
function pretty_print(colors::Vector{UInt8})
	@assert length(colors) == 5 || length(colors) == 8
	if length(colors) == 5
		print("$NOBLE_1C<+3> ")
		for (idx, req) in enumerate(colors)	
			if req != 0
				print("$(req)$(GEM_COLORS[idx]) ")	
			end
		end
		println("")
	elseif colors[COLORIDX] != 0
		print("$(CARD_COLORS[colors[COLORIDX]]) Card<+$(colors[SCOREIDX])> ")
			for (idx, req) in enumerate(colors[1:5])	
				if req != 0
					print("$(req)$(GEM_COLORS[idx]) ")	
				end
			end
		println("")
		else 
			println("<已售罄>")
		end
	
end

function show_cards_nobles(nobles::Vector{Vector{UInt8}})
	foreach(n -> pretty_print(n), nobles)	
end

function show_cards_nobles(deck::Matrix{Vector{UInt8}}, turn)
	println("ROUND ($turn)\t")
	split_by_level(1)
	@fastmath  @inbounds foreach(c -> pretty_print(c), deck[:, 1])
	split_by_level(2)
	@fastmath  @inbounds foreach(c -> pretty_print(c), deck[:, 2])
	split_by_level(3)
	@fastmath  @inbounds foreach(c -> pretty_print(c), deck[:, 3])

end

function show_available_gems(gems::Vector{UInt8})
	split_row()	
	print("AVAILABLE GEMS >>")
	for (idx, num) in enumerate(gems)
		print(" ($(GEM_COLORS[idx])):$num ")
	end
	println("")
end

function show_players(p0::Player, p1::Player)
	println("|===========P0[$(p0.scores)]==========><==========[$(p1.scores)]P1===========|")
	for idx in 1:5
		println("|            $(p0.gems[idx])            $(GEM_COLORS[idx])           $(p1.gems[idx])	       |")
		# |===========P1===========><===========P2===========|
		# |           0            🟢           0            |
	end
	print(" | ")
	for idx in 1:5
		print("$(p0.bought[idx])$(CARD_COLORS[idx])  ")
	end
	print(" || ")
	for idx in 1:5
		print("$(p1.bought[idx])$(CARD_COLORS[idx])  ")
	end
	println("")
	split_row()
end

# TODO: 改成宏
function split_by_level(lv)
	println("=[$lv 级卡]=======================================================")
end


function split_row()
	println("=================================================================")
end

"返回字符串的解析指令"
function similar_cmd(input::String)

	# parse

	# 相似指令 推荐

end

"Based on Hamming Distance"
function str_similar_index(a::String, b::String)
	if length(a) == 0 && length(b) == 0
		1
	end
	if length(a) == 0 || length(b) == 0
		0
	end
	if length(a) != length(b)
		len_coff = 0.88
	else
		len_coff = 1
	end
	# avec = Vector{Char}(a)
	# bvec = Vector{Char}(b)
	aset = Set(a)
	bset = Set(b)
	intersection_num = length(aset ∩ bset)
	if intersection_num == 0
		0
	end
	union_num = length(aset ∪ bset)
	float((intersection_num / union_num)*len_coff)
end

"返回卡牌的(level, idx)"
function parse_buy(command::RegexMatch)
	nums = command.captures[1]
	nums_vec = split(nums, ',')
	@assert length(nums_vec) == 2  "Buy Level,Id"
	@. parse(UInt8, nums_vec)
end

function parse_pick(command::RegexMatch)
	nums = strip(command.captures[1], [' ', ','])  # strip the ' ' and ',' at the head&tail of captured string 
	nums_split = split(nums, ',')
	len = length(nums_split)
	@assert len == 2 || len == 3 "指令长度异常"
	@assert isdigit(nums_split[1][1]) && isdigit(nums_split[2][1]) "参数须为数" 
	if len == 3
		@assert isdigit(nums_split[3][1]) "第三参非数"
	end
	@. parse(UInt8, nums_split)
end