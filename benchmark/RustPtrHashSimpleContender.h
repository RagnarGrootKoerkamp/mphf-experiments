#pragma once

#include "Contender.h"
#include "RustFmphContender.h"

#include <cstddef>

extern "C" {
void* createPtrHashStruct(uint64_t len, const char** str);
void constructPtrHash(void* rustStruct, bool compact);
uint64_t queryPtrHash(void* rustStruct, const char* key, const size_t length);
uint64_t queryPtrHashStream(void* rustStruct, const char** const keys, const size_t* lengths,
                            const size_t num_keys);
size_t sizePtrHash(void* rustStruct);
void destroyPtrHashStruct(void* rustStruct);
void initializeRayonThreadPool(uint64_t threads);
}

class RustPtrHashContender : public Contender {
  protected:
	void* rustStruct = nullptr;
	const char** data;
	bool compact;

  public:
	RustPtrHashContender(size_t N, bool compact) : Contender(N, 1.0), compact(compact) {
		data =
		    static_cast<const char**>(malloc(std::max(N, Contender::numQueries) * sizeof(char*)));
	}

	~RustPtrHashContender() override {
		if(rustStruct != nullptr) {
			destroyPtrHashStruct(rustStruct);
		}
		free(data);
	}

	std::string name() override {
		return std::string("PtrHash") + " params=" + (compact ? "compact" : "fast");
	}

	void beforeConstruction(const std::vector<std::string>& keys) override {
		std::cout << "Converting input" << std::endl;
		for(size_t i = 0; i < N; i++) {
			data[i] = keys[i].c_str();
		}
		std::cout << "Sending to Rust" << std::endl;
		if(!rayonThreadsInitialized) {
			rayonThreadsInitialized = true;
			initializeRayonThreadPool(numThreads);
		}
		std::cout << "N: " << N << " len " << keys.size() << std::endl;
		rustStruct = createPtrHashStruct(N, data);
	}

	void construct(const std::vector<std::string>& keys) override {
		(void)keys;
		constructPtrHash(rustStruct, compact);
	}

	size_t sizeBits() override { return sizePtrHash(rustStruct) * 8; }

	void performQueries(const std::span<std::string> keys) override {
		auto x = [&](std::string& key) {
			return queryPtrHash(rustStruct, key.c_str(), key.length());
		};
		doPerformQueries(keys, x);
	}

	void performTest(const std::span<std::string> keys) override {
		auto x = [&](std::string& key) {
			return queryPtrHash(rustStruct, key.c_str(), key.length());
		};
		doPerformTest(keys, x);
	}
};

class RustPtrHashContenderStream : public RustPtrHashContender {
	const char** queries;
	size_t* queryLengths;

  public:
	RustPtrHashContenderStream(size_t N, bool compact) : RustPtrHashContender(N, compact) {}

	std::string name() override {
		return std::string("PtrHash-streaming") + " params=" + (compact ? "compact" : "fast");
	}

	void beforeQueries(const std::span<std::string> keys) override {
		queries      = static_cast<const char**>(malloc(keys.size() * sizeof(char*)));
		queryLengths = static_cast<size_t*>(malloc(keys.size() * sizeof(size_t)));
		for(size_t i = 0; i < keys.size(); i++) {
			queries[i]      = keys[i].c_str();
			queryLengths[i] = keys[i].length();
		}
	}

	void performQueries(const std::span<std::string> keys) override {
		auto sum = queryPtrHashStream(rustStruct, queries, queryLengths, keys.size());
		DO_NOT_OPTIMIZE(sum);
	}
};
