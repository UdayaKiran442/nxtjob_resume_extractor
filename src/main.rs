use rust_bert::pipelines::ner::NERModel;
use std::path::Path;
use tch::Device;

fn main() {
    let config_path =
        Path::new("/Users/uday/Desktop/bert-large-cased-finetuned-conll03-english/config.json");
    let vocab_path =
        Path::new("/Users/uday/Desktop/bert-large-cased-finetuned-conll03-english/vocab.txt");
    let weights_path =
        Path::new("/Users/uday/Desktop/bert-large-cased-finetuned-conll03-english/rust_model.ot");

    let ner_model = NERModel::new(vocab_path, config_path, weights_path, Device::Cpu)
        .expect("Failed to create NER Model");

    let input = [
        "Gonuguntla Udaya Kiran 9160891919 | gudaya2002@gmail.com | LinkedIn | GitHub | Portfolio Internship Experience Teaching Assistant in NodeJS January 2023 – March 2023 Coding Ninjas Remote • Guided students through practical Node.js projects, offering real-time feedback and tailored support; this approach led to a significant improvement in student understanding, with a 30% increase in project completion rates. • Demonstrated exceptional problem-solving abilities by crafting and delivering customized technical content to students, resulting in an impressive 95% satisfaction rate and notably improved comprehension and retention of course material. Full Stack Developer May 2022 – July 2022 Spotflock Technologies Hyderabad, Telangana • Crafted a sophisticated e-commerce shopping cart solution with predictive cart abandonment prevention and one-click reorder capability; streamlined the purchasing journey, resulting in a 40% increase in mobile conversion rates and a 25% decrease in customer service inquiries. • Optimized website performance by implementing code refactoring techniques, resulting in a 20% improvement in page load speed. Technical Skills Languages : Java, JavaScript, HTML/CSS Databases : SQL(MySQL, PostgreSQL), NoSQL(MongoDB, Firebase),Neon Libraries : ReactJs, Web3.js Frameworks : React Native, ExpressJs, NestJs, NextJs, HonoJs Developer Tools : Git, VS Code, Postman, NeoVim, tmux DevOps : AWS, Linux, Docker, Kubernetes, Jenkins, CI/CD Projects Bus Booking API | NestJs, PostgreSQL, AWS EC2, AWS RDS, AWS ALB April 2023 - May 2023 • Engineered robust bus booking API’s with NestJS, achieving 100% data consistency. • Directed the deployment of PostgreSQL on AWS RDS, integrated read replicas and standby databases to enhance performance and fault tolerance, fortifying a scalable architecture that boosted system reliability and response time by 35% . Student Management App | NodeJs, React Native, Docker, AWS February 2023 - April 2023 • Engineered a proprietary app for universities to streamline administrative tasks, leading to a 30% reduction in workload, enhanced productivity, and facilitated seamless operations • Directed the establishment of a seamless CI/CD pipeline on AWS, enhancing deployment speed by 50% and enabling rapid, error-free releases, resulting in a 20% increase in software quality and customer retention. Quiz Portal | NodeJs, MongoDB, ReactJs, ReduxJs August 2022 - November 2022 • Implemented an admin panel feature on the quiz website, streamlining exam administration and student progress tracking for teachers; resulted in a 30% boost in exam efficiency and student performance metrics. • Increased user engagement by 40% through the creation of interactive features within the quiz website, such as real-time performance tracking. Education BML Munjal University Gurugram, Haryana B.Tech in Computer Science August 2020 – July 2024 Happy Valley School Agiripalli, Andhra Pradesh 12th Class April 2018 – March 2020 Co-Circular Activities Engaged in chess events and club tournaments organized by private chess clubs in the DELHI-NCR region, showcasing active participation and competitive spirit."
    ];

    let output = ner_model.predict(&input);
    println!("{:?}", output);
    println!("Hello, world!");
}
