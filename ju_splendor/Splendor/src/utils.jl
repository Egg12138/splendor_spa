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
	"我们不再使用玩家id来存储行动顺序的信息，我们改记为行动次数，这样也方便做反馈"
	actcounter::UInt8
	"在这里，我们对规则做出调整，我们只限制总的宝石不能超过10个，把黄金排除宝石之列，这样可以少建一些字段"
	gems::Vector{UInt8}
	"发展卡颜色数: bought[1] => 绿卡数, bought[2] => 白卡数... 1,2,3,4,5 => GREEN, WHITE, BLUE, BLACK, RED"
	bought::Vector{UInt8}
	golds::UInt8
	scores::UInt8
	"在第一次比较平分时，发展卡数量更少的玩家获胜"
	cards_num::UInt8
	reserved_cards::Vector{UInt8}
	reserved_num::UInt8
end

PlayerInitial(cnt) = Player(cnt, clrmap(), clrmap(), 0, 0, 0, res_init(), 0)
function Increment(p::Player, score)  
	p.score += score
end

a = 0
using Random
# 判定函数：
can_uncover(deck::Vector{Vector{UInt8}}) = length(deck) != 0
can_reserve(player::Player) = length(player.reserved_num) < MAX_RESERVE
reach_target(player::Player) = player.scores >= TARGET
enough(n) = n > 0 
# turn_over(p1::Player, p2::Player) = p1.actcounter == p2.actcounter
# 后手有一点点的微弱补偿
function winner(p1::Player, p2::Player)
	( p1.score > p2.score) ?
	 ( p1 ) : ( (p1.score < p2.score) ? 
	 				( p2 ) : ( p1.cards_num < p2.cards_num ) ? (p1) : (p2) )
end

function handle_noble_sponse!(player::Player, nobles_list::Vector{Vector{UInt8}})
	for (id, noble) in enumerate(nobles_list)
		delta_vec = _Δrequirement(player, noble)
		if sum(delta_vec) >= 0
			println("You won noble[$id]")
			won_a_noble(player)
			deleteat!(nobles_list, id)
		end
	end

end

# 暂时不考虑黄金
"一种解决办法是提供给玩家有限的支付方式的选项，但是这样让应该会AI比较难受"
function affort(player::Player, card::Vector{UInt8})
	delta = _Δrequirement(player.gems, card)
	sum(delta) <= 0 
end

# private functions
function _Δrequirement(data::Vector{UInt8}, req::Vector{UInt8})
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


"只考虑了玩家自己的状态"
function max_pick_num(player)
	(player.gems <= 7) ? 3 : (MAX_GEMS - player.gems )
end
# 状态机更新
# 引入动作
@enum Action OneColor=1 DiffColor=2 Buy=3 Reserve=4

gems_area_reset() = 7 * ones(UInt8, 5)
function won_a_noble(player::Player)
	increment(player, NOBLE_BONUS)
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
"""
function _remove_one_from_store!(store, level, idx)
	store[idx, level] = zeros(UInt8, 8)
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
	println("$(d1) = $(d2) = $(d3)")
	
	level1 = uncover4!(d1)
	level2 = uncover4!(d2)
	level3 = uncover4!(d3)
	[level1 level2 level3]
end
"Color map vector"
clrmap() = zeros(UInt8, 5)
res_init() = zeros(UInt8, MAX_RESERVE)
global_gems_init() = 7 * ones(UInt8, 5)

function take_action(player::Player)
	#IMPL: Finish
end

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
	foreach(c -> pretty_print(c), deck[:, 1])
	split_by_level(2)
	foreach(c -> pretty_print(c), deck[:, 2])
	split_by_level(3)
	foreach(c -> pretty_print(c), deck[:, 3])

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
	println("|===========P1===========><===========P2===========|")
	for idx in 1:5
		println("|	    $(p0.gems[idx])            $(GEM_COLORS[idx])           $(p1.gems[idx])	  	  |")
		# |===========P1===========><===========P2===========|
		# |           0            🟢           0            |
	end
	for idx in 1:5
		print("$(p0.bought[idx])$(CARD_COLORS[idx])  ")
	end
	print(" | ")
	for idx in 1:5
		print("$(p1.bought[idx])$(CARD_COLORS[idx])  ")
	end
	println("")
	split_row()
end

# TODO: 改成宏
function split_by_level(lv)
	println("=[$lv 级卡]==========================================================")
end


function split_row()
	println("===================================================================")
end

