(defproject speicherstadt "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :min-lein-version "2.0.0"
  :dependencies [[org.clojure/clojure "1.8.0"]
                 [com.stuartsierra/component "0.3.1"]
                 [compojure "1.4.0"]
                 [duct "0.5.7"]
                 [environ "1.0.1"]
                 [meta-merge "0.1.1"]
                 [me.raynes/fs "1.4.6"]
                 [ring "1.4.0"]
                 [ring/ring-defaults "0.1.5"]
                 [ring-jetty-component "0.3.0"]
                 [ring/ring-json "0.4.0"]]
  :plugins [[lein-environ "1.0.1"]
            [lein-gen "0.2.2"]]
  :generators [[duct/generators "0.3.0"]]
  :duct {:ns-prefix speicherstadt}
  :main ^:skip-aot speicherstadt.main
  :target-path "target/%s/"
  :aliases {"gen"   ["generate"]
            "setup" ["do" ["generate" "locals"]]}
  :profiles
  {:dev  [:project/dev  :profiles/dev]
   :test [:project/test :profiles/test]
   :uberjar {:aot :all}
   :profiles/dev  {}
   :profiles/test {}
   :project/dev   {:source-paths ["dev"]
                   :repl-options {:init-ns user}
                   :dependencies [[reloaded.repl "0.2.1"]
                                  [org.clojure/tools.namespace "0.2.11"]
                                  [kerodon "0.7.0"]]
                   :env {:port 3000}}
   :project/test  {}})
