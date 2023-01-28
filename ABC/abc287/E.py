# トライ
# https://ikatakos.com/pot/programming_algorithm/data_structure/trie
class BinaryTrie:
 
    def __init__(self, bit_depth):
        self.root = [None, None, 0]  # [0-child, 1-child, count]
        self.bit_start = 1 << (bit_depth - 1)
        self.xor_mask = 0
 
    def insert(self, x):
        """xを格納"""
        b = self.bit_start
        node = self.root
        node[2] += 1
        while b:
            i = bool(x & b)
            if node[i] is None:
                node[i] = [None, None, 1]
            else:
                node[i][2] += 1
            node = node[i]
            b >>= 1
 
    def pop_min(self):
        """xor_mask適用後の最小値を、適用前の値で取得し、木からは削除"""
        b = self.bit_start
        node = self.root
        m = self.xor_mask
        ret = 0
        node[2] -= 1
        while b:
            i = bool(m & b)
            if node[i] is None:
                i ^= 1
            ret = (ret << 1) + i
 
            if node[i][2] > 1:
                node[i][2] -= 1
                node = node[i]
            else:
                tmp = node[i]
                node[i] = None
                node = tmp
            b >>= 1
        return ret


        