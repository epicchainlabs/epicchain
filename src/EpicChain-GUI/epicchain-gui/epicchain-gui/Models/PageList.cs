using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models
{
    public class PageList<T>
    {
        public int PageIndex { get; set; }
        public int PageSize { get; set; }
        public int TotalCount { get; set; }
        public List<T> List { get; set; } = new List<T>();


        public PageList<TTarget> Project<TTarget>(Func<T, TTarget> project)
        {
            var model = new PageList<TTarget>()
            {
                PageSize = this.PageSize,
                PageIndex = this.PageIndex,
                TotalCount = this.TotalCount,
                List = this.List.Select(project).ToList(),
            };
            return model;
        }
    }
}
