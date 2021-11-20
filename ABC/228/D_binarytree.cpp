#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
#define rep(i, n) for (int i = 0; i < (int)(n); i++)

const ll N = 1 << 20;

// binaryTree: https://www.delftstack.com/ja/howto/cpp/binary-tree-insert-in-cpp/
struct TreeNode {
    int key;
    struct TreeNode *left{};
    struct TreeNode *right{};
};

void insertNode(TreeNode *&root, const int k) {
    if (root == nullptr) {
        root = new TreeNode;
        root->key = k;
        root->left = nullptr;
        root->right = nullptr;
    } else {
        if (k < root->key)
            insertNode(root->left, k);
        else
            insertNode(root->right, k);
    }
}

TreeNode *findNode(TreeNode *root, const int k) {
    if (root == nullptr)
        return nullptr;

    if (k == root->key)
        return root;

    if (k < root->key)
        return findNode(root->left, k);
    else
        return findNode(root->right, k);
}


int main() {
    vector<int> v1 {11, 23, 3, 5, 9, 15, 2, 20};

    TreeNode *root = nullptr;

    for (const auto &item : v1) {
        insertNode(root, item);
    }


    vector<int> v2 {1, 22, 4, 16};
    for (const auto &item : v2) {
        insertNode(root, item);
    }

    printf("findNode(root, 2) = %d\n", findNode(root, 2));

}