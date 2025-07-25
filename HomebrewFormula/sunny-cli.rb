class SunnyCli < Formula
	desc "Check the weather from your terminal."
	homepage "https://github.com/jamesukiyo/sunny-rs"
	license "MIT"

	on_macos do
		if Hardware::CPU.intel?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-darwin-x64.tar.gz"
			sha256 "0cc0f7aefcc3b44ea99f9e1bb66ed990e4930c62f1efa6e5d952ebae4ad3b516"
		end
		if Hardware::CPU.arm?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-darwin-arm64.tar.gz"
			sha256 "7e3d6fabd6ec80aa83e0858248f43026fc5f3c1f10e582fa408c1105c6d165cd"
		end
	end

	on_linux do
		if Hardware::CPU.intel?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-linux-x64.tar.gz"
			sha256 "fed36b614f47bf05adb2772b66e0a72e068f1432f9d7dd1931351c9309e98a23"
		end
		if Hardware::CPU.arm?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-linux-arm64.tar.gz"
			sha256 "5b653060f38273c8e46e69db6ccacc5f6ccf8ec11fd7c334fce20f9c85153db7"
		end
	end

	def install
		bin.install "sunny"
	end

	test do
		system "#{bin}/sunny", "--version"
	end
end
