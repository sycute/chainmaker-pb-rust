build:
	git clone -b v2.3.3 https://git.chainmaker.org.cn/chainmaker/pb.git --depth 1 proto
	cargo build -r
	rm -rf proto