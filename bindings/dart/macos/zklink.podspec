framework_name = 'zklink_sdk_dart'
url = "https://github.com/zkLinkProtocol/zklink_sdk/releases/download/dart_sdk_0.0.1/aarch64-apple-darwin.tar.xz"
archive = "#{framework_name}.tar.xz"
`
mkdir -p Frameworks/#{framework_name}
cd Frameworks/#{framework_name}

if [ ! -f #{archive} ]
then
  curl -L #{url} -o #{archive}
fi

tar xvf #{archive}
cd -
`

Pod::Spec.new do |spec|
  spec.name          = 'zklink_sdk_dart'
  spec.version       = '0.0.1'
  spec.license       = { :file => '../LICENSE' }
  spec.homepage      = 'https://zk.link'
  spec.summary       = 'zkLink Dart SDK'

  spec.source              = { :git => 'https://github.com/zkLinkProtocol/zklink_sdk.git' }
  spec.vendored_frameworks = "Frameworks/#{framework_name}"

  spec.ios.deployment_target = '11.0'
  spec.osx.deployment_target = '10.14'
end